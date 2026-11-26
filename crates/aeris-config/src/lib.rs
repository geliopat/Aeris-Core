#![forbid(unsafe_code)]

use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
const BUILTINS: &[&str] = &[
    "DIRECT",
    "REJECT",
    "REJECT-DROP",
    "PASS",
    "GLOBAL",
    "ZAPRET",
    "DIRECT-NO-ZAPRET",
];
const GROUP_TYPES: &[&str] = &[
    "select",
    "url-test",
    "fallback",
    "load-balance",
    "smart",
    "adaptive",
];
const PROTOCOLS: &[&str] = &[
    "vless",
    "vmess",
    "trojan",
    "ss",
    "ss2022",
    "hysteria2",
    "tuic",
    "anytls",
    "wireguard",
    "socks5",
    "http",
    "naive",
];

/// Diagnostics intentionally omit input values, paths and third-party error text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Read,
    TooLarge,
    Encoding,
    Syntax,
    Shape(String),
    Reference(String),
    Cycle,
    DuplicateName,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => f.write_str("cannot read configuration"),
            Self::TooLarge => f.write_str("configuration exceeds input size limit"),
            Self::Encoding => f.write_str("configuration is not UTF-8"),
            Self::Syntax => f.write_str("invalid, duplicate, or unsupported YAML; input omitted"),
            Self::Shape(field) => write!(f, "invalid structure at {field}"),
            Self::Reference(field) => write!(f, "unresolved reference at {field}"),
            Self::Cycle => f.write_str("cyclic proxy/group dependency"),
            Self::DuplicateName => f.write_str("duplicate or reserved proxy/group name"),
        }
    }
}

impl std::error::Error for Error {}

/// This is a structural validation result, not a runtime-compatible configuration.
/// No field is applied, normalized, serialized back, or used to open a connection.
pub struct ConfigDocument {
    source: String,
    parsed: Map<String, Value>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CheckReport {
    pub unchecked_top_level_fields: usize,
    pub unchecked_rule_matchers: usize,
    pub rules_after_match: usize,
}

fn shape(path: &str) -> Error {
    Error::Shape(path.to_owned())
}

fn decode(text: &str) -> Result<Value, Error> {
    if text.len() > MAX_INPUT_BYTES {
        return Err(Error::TooLarge);
    }
    let mut budget = serde_saphyr::Budget::default();
    budget.max_inclusion_depth = 0;
    budget.max_documents = 1;
    budget.max_depth = 48;
    budget.flow_nesting_limit = 48;
    budget.max_total_scalar_bytes = 32 * 1024 * 1024;
    let mut options = serde_saphyr::Options::default();
    options.budget = Some(budget);
    options.reject_unsupported_tags = true;
    options.reject_tagged_field_names = true;
    options.reject_non_finite_typeless_float = true;
    options.with_snippet = false;
    serde_saphyr::from_str_with_options(text, options).map_err(|_| Error::Syntax)
}

fn mapping<'a>(value: &'a Value, path: &str) -> Result<&'a Map<String, Value>, Error> {
    value.as_object().ok_or_else(|| shape(path))
}

fn sequence<'a>(value: &'a Value, path: &str) -> Result<&'a [Value], Error> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| shape(path))
}

fn list<'a>(map: &'a Map<String, Value>, field: &str) -> Result<&'a [Value], Error> {
    match map.get(field) {
        Some(value) => sequence(value, field),
        None => Ok(&[]),
    }
}

fn nonempty<'a>(value: &'a Value, path: &str) -> Result<&'a str, Error> {
    value
        .as_str()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| shape(path))
}

fn required<'a>(
    map: &'a Map<String, Value>,
    field: &str,
    path: &str,
) -> Result<&'a str, Error> {
    nonempty(map.get(field).ok_or_else(|| shape(path))?, path)
}

fn enum_field(map: &Map<String, Value>, field: &str, allowed: &[&str]) -> Result<(), Error> {
    if let Some(value) = map.get(field) {
        let value = nonempty(value, field)?;
        if !allowed.contains(&value) {
            return Err(shape(field));
        }
    }
    Ok(())
}

fn bool_field(map: &Map<String, Value>, field: &str) -> Result<(), Error> {
    if map.get(field).is_some_and(|v| !v.is_boolean()) {
        return Err(shape(field));
    }
    Ok(())
}

fn string_list(map: &Map<String, Value>, field: &str) -> Result<(), Error> {
    for value in list(map, field)? {
        nonempty(value, field)?;
    }
    Ok(())
}

fn port(value: &Value, path: &str, allow_zero: bool) -> Result<(), Error> {
    let min = u64::from(!allow_zero);
    if value
        .as_u64()
        .is_none_or(|n| !(min..=65_535).contains(&n))
    {
        return Err(shape(path));
    }
    Ok(())
}

impl ConfigDocument {
    pub fn read(path: &Path) -> Result<Self, Error> {
        let file = File::open(path).map_err(|_| Error::Read)?;
        let mut bytes = Vec::new();
        file.take((MAX_INPUT_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|_| Error::Read)?;
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(Error::TooLarge);
        }
        let text = String::from_utf8(bytes).map_err(|_| Error::Encoding)?;
        Self::parse(text)
    }

    pub fn parse(source: String) -> Result<Self, Error> {
        let Value::Object(parsed) = decode(&source)? else {
            return Err(shape("root"));
        };
        if parsed.is_empty() {
            return Err(shape("root"));
        }
        Ok(Self { source, parsed })
    }

    /// Read-only original text, including comments and whitespace.
    pub fn original(&self) -> &str {
        &self.source
    }

    pub fn check(&self) -> Result<CheckReport, Error> {
        let root = &self.parsed;
        let mut report = CheckReport::default();
        enum_field(root, "mode", &["rule", "global", "direct"])?;
        for field in ["ipv6", "allow-lan"] {
            bool_field(root, field)?;
        }
        for field in [
            "port",
            "socks-port",
            "mixed-port",
            "redir-port",
            "tproxy-port",
        ] {
            if let Some(value) = root.get(field) {
                port(value, field, true)?;
            }
        }
        if let Some(value) = root.get("tun") {
            let tun = mapping(value, "tun")?;
            enum_field(tun, "stack", &["system", "rips", "mixed", "auto"])?;
            for field in ["enable", "auto-route", "strict-route", "auto-detect-interface"] {
                bool_field(tun, field)?;
            }
            for field in ["dns-hijack", "route-exclude-address"] {
                string_list(tun, field)?;
            }
        }
        if let Some(value) = root.get("dns") {
            let dns = mapping(value, "dns")?;
            enum_field(dns, "enhanced-mode", &["fake-ip", "redir-host", "hybrid"])?;
            for field in ["enable", "ipv6", "respect-rules"] {
                bool_field(dns, field)?;
            }
            for field in [
                "nameserver",
                "default-nameserver",
                "proxy-server-nameserver",
                "direct-nameserver",
                "fallback",
                "fake-ip-filter",
            ] {
                string_list(dns, field)?;
            }
        }
        let providers = provider_names(root, "proxy-providers")?;
        let rule_providers = provider_names(root, "rule-providers")?;
        let mut graph = BTreeMap::<&str, Vec<&str>>::new();
        for proxy in list(root, "proxies")? {
            let p = mapping(proxy, "proxies[]")?;
            let name = required(p, "name", "proxies[].name")?;
            let kind = required(p, "type", "proxies[].type")?;
            if !PROTOCOLS.contains(&kind) {
                return Err(shape("proxies[].type"));
            }
            required(p, "server", "proxies[].server")?;
            port(
                p.get("port").ok_or_else(|| shape("proxies[].port"))?,
                "proxies[].port",
                false,
            )?;
            let mut edges = Vec::new();
            if let Some(dialer) = p.get("dialer-proxy") {
                edges.push(nonempty(dialer, "proxies[].dialer-proxy")?);
            }
            add_node(&mut graph, name, edges)?;
        }
        for group in list(root, "proxy-groups")? {
            let g = mapping(group, "proxy-groups[]")?;
            let name = required(g, "name", "proxy-groups[].name")?;
            let kind = required(g, "type", "proxy-groups[].type")?;
            if !GROUP_TYPES.contains(&kind) {
                return Err(shape("proxy-groups[].type"));
            }
            let mut edges = Vec::new();
            for member in list(g, "proxies")? {
                edges.push(nonempty(member, "proxy-groups[].proxies[]")?);
            }
            for provider in list(g, "use")? {
                let name = nonempty(provider, "proxy-groups[].use[]")?;
                if !providers.contains(name) {
                    return Err(Error::Reference("proxy-groups[].use[]".to_owned()));
                }
            }
            add_node(&mut graph, name, edges)?;
        }
        check_graph(&graph)?;
        let mut matched = false;
        for rule in list(root, "rules")? {
            let rule = nonempty(rule, "rules[]")?;
            if matched {
                report.rules_after_match += 1;
            }
            let mut parts = rule.split(',').map(str::trim);
            match parts.next() {
                Some("MATCH") => {
                    let target = parts.next().ok_or_else(|| shape("rules[].MATCH"))?;
                    if target.is_empty() || parts.next().is_some() {
                        return Err(shape("rules[].MATCH"));
                    }
                    check_target(&graph, target)?;
                    matched = true;
                }
                Some(
                    "DOMAIN" | "DOMAIN-SUFFIX" | "DOMAIN-KEYWORD" | "IP-CIDR" | "IP-CIDR6"
                    | "GEOIP" | "GEOSITE" | "IP-ASN" | "PROCESS-NAME" | "PROCESS-PATH"
                    | "RULE-SET",
                ) => {
                    let matcher = parts.next().ok_or_else(|| shape("rules[]"))?;
                    let target = parts.next().ok_or_else(|| shape("rules[]"))?;
                    if matcher.is_empty() || target.is_empty() {
                        return Err(shape("rules[]"));
                    }
                    if rule.starts_with("RULE-SET,") && !rule_providers.contains(matcher) {
                        return Err(Error::Reference("rules[].RULE-SET".to_owned()));
                    }
                    check_target(&graph, target)?;
                }
                _ => report.unchecked_rule_matchers += 1,
            }
        }
        const CHECKED: &[&str] = &[
            "mode",
            "ipv6",
            "allow-lan",
            "port",
            "socks-port",
            "mixed-port",
            "redir-port",
            "tproxy-port",
            "tun",
            "dns",
            "proxies",
            "proxy-groups",
            "proxy-providers",
            "rule-providers",
            "rules",
        ];
        report.unchecked_top_level_fields = root
            .keys()
            .filter(|k| !CHECKED.contains(&k.as_str()))
            .count();
        Ok(report)
    }

    /// User rules stay separate; this only previews order and never applies routing.
    pub fn preview_rules<'a>(&'a self, user: &'a UserRules) -> impl Iterator<Item = &'a str> {
        user.rules.iter().map(String::as_str).chain(
            self.parsed
                .get("rules")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str),
        )
    }
}

fn provider_names<'a>(
    root: &'a Map<String, Value>,
    field: &str,
) -> Result<BTreeSet<&'a str>, Error> {
    let mut result = BTreeSet::new();
    if let Some(value) = root.get(field) {
        for (name, body) in mapping(value, field)? {
            if name.trim().is_empty() {
                return Err(shape(field));
            }
            mapping(body, field)?;
            result.insert(name.as_str());
        }
    }
    Ok(result)
}

fn add_node<'a>(
    graph: &mut BTreeMap<&'a str, Vec<&'a str>>,
    name: &'a str,
    edges: Vec<&'a str>,
) -> Result<(), Error> {
    if BUILTINS.contains(&name) || graph.insert(name, edges).is_some() {
        return Err(Error::DuplicateName);
    }
    Ok(())
}

fn check_target(graph: &BTreeMap<&str, Vec<&str>>, target: &str) -> Result<(), Error> {
    if graph.contains_key(target) || BUILTINS.contains(&target) {
        Ok(())
    } else {
        Err(Error::Reference("rules[].target".to_owned()))
    }
}

fn check_graph(graph: &BTreeMap<&str, Vec<&str>>) -> Result<(), Error> {
    let mut indegree: BTreeMap<&str, usize> = graph.keys().map(|&name| (name, 0)).collect();
    for edges in graph.values() {
        for &edge in edges {
            if let Some(count) = indegree.get_mut(edge) {
                *count += 1;
            } else if !BUILTINS.contains(&edge) {
                return Err(Error::Reference("proxy/group dependency".to_owned()));
            }
        }
    }
    let mut queue: VecDeque<&str> = indegree
        .iter()
        .filter_map(|(&name, &count)| (count == 0).then_some(name))
        .collect();
    let mut visited = 0;
    while let Some(name) = queue.pop_front() {
        visited += 1;
        for edge in &graph[name] {
            if let Some(count) = indegree.get_mut(edge) {
                *count -= 1;
                if *count == 0 {
                    queue.push_back(edge);
                }
            }
        }
    }
    if visited != graph.len() {
        return Err(Error::Cycle);
    }
    Ok(())
}

/// Separate ordered user-rule document, never a provider-YAML override.
pub struct UserRules {
    rules: Vec<String>,
}

impl UserRules {
    pub fn parse(source: &str) -> Result<Self, Error> {
        let value = decode(source)?;
        let mut rules = Vec::new();
        for item in sequence(&value, "user-rules")? {
            let rule = nonempty(item, "user-rules[]")?;
            if !rule.contains(',') {
                return Err(shape("user-rules[]"));
            }
            rules.push(rule.to_owned());
        }
        Ok(Self { rules })
    }
}
