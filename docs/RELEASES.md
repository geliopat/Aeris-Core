# Release and Nightly — gates

Independent versions start at 0.1.0. Release requires manual approval and all requirements in TESTING.md. Nightly requires actual implemented functionality, passing mandatory CI, and a change since its previous build; use a unique UTC date/run identifier. Never publish unsigned manifests or unsupported driver packages. Initial CI only builds/tests the scaffold; no publication, signing, or automatic release workflow is enabled. Add release automation only after signing trust roots, artifact layout, rollback and real Windows validation are established. Preview channel is excluded.

## Approved details / Уточнения

- Release — канал по умолчанию, ручная публикация после полного gate; Nightly — автоматическая сборка/публикация при изменениях после готовности безопасного pipeline и прохождения обязательных проверок. Предрелизная SemVer форма `1.1.0-nightly.20261126` — согласованный пример, не обещанная версия; уникальность при нескольких сборках в дату обеспечить отдельным run identifier/полем манифеста либо документированной совместимой схемой, без неоднозначного сравнения версий.
- Возможность явного возврата Nightly → Release, если безопасна миграция; при несовместимости остановиться с диагностикой. Не сбрасывать настройки молча.
- Проверка обновлений каждые 24 ч; скачивать только изменившиеся компоненты; установка клиента/ядра по подтверждению, ресурсы по настройкам, драйверы с подтверждением. Раздельные совместимые версии клиента, ядра, службы, API/IPC v1+, ресурсных наборов и драйверов. Отдельные подписанные манифесты каналов; альтернативный URL не меняет доверенные ключи. Atomic rollback.
- Текущие доказательства CI и границы реализованного — [STATUS.md](STATUS.md). Успешная сборка каркаса не разрешает выпуск Release 1.0.0.
