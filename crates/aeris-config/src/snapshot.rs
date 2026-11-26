use super::{CheckReport, ConfigDocument, Error, UserRules};
use std::fmt;
use std::sync::{Arc, RwLock};

/// A checked immutable profile candidate, NOT an active network configuration.
pub struct ConfigSnapshot {
    generation: u64,
    document: Arc<ConfigDocument>,
    user_rules: Arc<UserRules>,
    report: Arc<CheckReport>,
}

impl ConfigSnapshot {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn original(&self) -> &str {
        self.document.original()
    }

    pub fn report(&self) -> &CheckReport {
        &self.report
    }

    pub fn rules(&self) -> impl Iterator<Item = &str> {
        self.document.preview_rules(&self.user_rules)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StoreError {
    Invalid(Error),
    Conflict,
    NoPrevious,
    Unavailable,
    GenerationExhausted,
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(error) => write!(f, "{error}"),
            Self::Conflict => f.write_str("profile generation changed"),
            Self::NoPrevious => f.write_str("no previous profile snapshot"),
            Self::Unavailable => f.write_str("profile store unavailable"),
            Self::GenerationExhausted => f.write_str("profile generation exhausted"),
        }
    }
}

impl std::error::Error for StoreError {}

struct State {
    current: Arc<ConfigSnapshot>,
    previous: Option<Arc<ConfigSnapshot>>,
}

/// In-memory transactional candidate storage. Does not activate networking,
/// write provider files, or persist history to disk. Readers retain old Arc
/// snapshots safely; publication and rollback use a generation precondition.
pub struct ProfileStore {
    state: RwLock<State>,
}

impl ProfileStore {
    pub fn new(document: ConfigDocument, user_rules: UserRules) -> Result<Self, StoreError> {
        let current = Self::candidate(1, document, user_rules)?;
        Ok(Self {
            state: RwLock::new(State {
                current,
                previous: None,
            }),
        })
    }

    fn candidate(
        generation: u64,
        document: ConfigDocument,
        user_rules: UserRules,
    ) -> Result<Arc<ConfigSnapshot>, StoreError> {
        let report = document
            .check_with_user_rules(Some(&user_rules))
            .map_err(StoreError::Invalid)?;
        Ok(Arc::new(ConfigSnapshot {
            generation,
            document: Arc::new(document),
            user_rules: Arc::new(user_rules),
            report: Arc::new(report),
        }))
    }

    pub fn current(&self) -> Result<Arc<ConfigSnapshot>, StoreError> {
        let state = self.state.read().map_err(|_| StoreError::Unavailable)?;
        Ok(Arc::clone(&state.current))
    }

    pub fn replace(
        &self,
        expected_generation: u64,
        document: ConfigDocument,
        user_rules: UserRules,
    ) -> Result<Arc<ConfigSnapshot>, StoreError> {
        let generation = expected_generation
            .checked_add(1)
            .ok_or(StoreError::GenerationExhausted)?;
        // Parse/check outside the write lock.
        let candidate = Self::candidate(generation, document, user_rules)?;
        let mut state = self.state.write().map_err(|_| StoreError::Unavailable)?;
        if state.current.generation != expected_generation {
            return Err(StoreError::Conflict);
        }
        let previous = std::mem::replace(&mut state.current, Arc::clone(&candidate));
        state.previous = Some(previous);
        Ok(candidate)
    }

    pub fn rollback(&self, expected_generation: u64) -> Result<Arc<ConfigSnapshot>, StoreError> {
        let mut state = self.state.write().map_err(|_| StoreError::Unavailable)?;
        if state.current.generation != expected_generation {
            return Err(StoreError::Conflict);
        }
        let generation = expected_generation
            .checked_add(1)
            .ok_or(StoreError::GenerationExhausted)?;
        let previous = state.previous.as_ref().ok_or(StoreError::NoPrevious)?;
        let restored = Arc::new(ConfigSnapshot {
            generation,
            document: Arc::clone(&previous.document),
            user_rules: Arc::clone(&previous.user_rules),
            report: Arc::clone(&previous.report),
        });
        state.current = Arc::clone(&restored);
        state.previous = None;
        Ok(restored)
    }
}
