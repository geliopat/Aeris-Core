# Контрольный реестр требований / Requirements traceability

Стабильные ID для задач и тестов. Это индекс согласованных требований, не замена полной спецификации и не checklist «готово». Не отмечать PASS целого ID пока не выполнены все его условия. В начале разработки все пункты OPEN/PARTIAL, кроме конкретных доказанных частей в STATUS. При реализации добавить к строке ссылку на commit/PR, tests/evidence, статус PASS/PARTIAL/FAIL/SKIPPED и ограничения. Детали и defaults по указанным источникам; поздние DECISIONS приоритетнее.

| ID | Обязательный результат | Источник / проверка |
|---|---|---|
| CORE-01 | Один Rust core, независимые жизненные циклы VPN/DPI/DNS; GUI и служба отдельно | SPECIFICATION §3–4, ARCHITECTURE §1–2; lifecycle tests |
| CORE-02 | FlClashX Mihomo assembly reference, Xray и иерархия DPI/AWG из UPSTREAM. Ядро нативное | UPSTREAM, COMPATIBILITY; provenance |
| CORE-03 | TCP/UDP IPv4/IPv6, Tokio, отмена, bounded queues, deadlines, backpressure, connection lifecycle | ARCHITECTURE §3; loopback/load/cancel |
| CORE-04 | rustls, fingerprint profiles, verified crypto, signed TLS data and rollback | ARCHITECTURE §2; interop/resource tests |
| PROTO-01 | VLESS REALITY/Vision/XHTTP/XUDP, все 4 XHTTP mode, XMUX, separate paths/settings | SPECIFICATION §3; Xray interop |
| PROTO-02 | VMess, Trojan, SS/2022, Hysteria2, TUIC, AnyTLS | COMPATIBILITY; оригинальные серверы |
| PROTO-03 | Обычный WireGuard и общая реализация актуальных и предыдущих официальных AmneziaWG, включая версии/обфускацию и явную диагностику. SOCKS5, HTTP/HTTPS, NaiveProxy H2/H3 в поддерживаемых режимах | COMPATIBILITY; interop |
| CFG-01 | Mihomo/Clash YAML subset, schema, type/reference/cycle/security validation, явные unsupported | CONFIGURATION §1–3,8 |
| CFG-02 | Read-only provider source, separate editable/reorderable/disableable user rules first | CONFIGURATION §1; immutable/order tests |
| CFG-03 | Atomic apply/rollback, last-good on error, safe live connections, history | CONFIGURATION §1; reload tests |
| CFG-04 | Local profiles/CLI; listeners/ports/LAN/controller/profile/geodata/SMUX/sub-rules supported subset | CONFIGURATION §2; compatibility fixtures |
| DNS-01 | UDP/TCP, DoH H1/H2/H3, DoT, DoQ, optional DNSSEC, explicit system only | ARCHITECTURE §4; leak/interop |
| DNS-02 | Policies, groups, hosts, #route, bootstrap, respect-rules, configured fallback, distinct DPI DNS | CONFIGURATION §5; routing/leak |
| DNS-03 | Fake-IP v4/v6, Redir-Host/Hybrid, stable mapping and exclusions, no fake outward | ARCHITECTURE §4; mapping/reload |
| DNS-04 | TTL/negative/prefetch/stale/coalescing/cache persistence, independent clear, benchmark choice | ARCHITECTURE §4; cache/bench |
| DNS-05 | DNS Always On/Proxy Access, DNS Kill Switch, UDP/TCP53 hijack v4/v6, optional encrypted endpoint block without MITM | CONFIGURATION §5; policy/leak |
| DNS-06 | TLS verification/pin optional/ECS/stamps/rebinding; pools, racing, recovery, per-module address family | ARCHITECTURE §4; security/bench |
| DNS-07 | Sniff HTTP Host/TLS SNI/QUIC without decrypt, ECH limitation; Happy Eyeballs/NAT64/DNS64 | ARCHITECTURE §3–4; tests |
| ROUTE-01 | First-match domain/IP/ASN/process/port/protocol/interface/logic/sub-rules; user before provider | CONFIGURATION §6; matcher/order |
| ROUTE-02 | DIRECT/proxy/REJECT/REJECT-DROP/PASS/ZAPRET/DIRECT-NO-ZAPRET, routing modes, no silent fallback | CONFIGURATION §6; policy |
| ROUTE-03 | select/url-test/fallback/load-balance/smart/adaptive; weights/sticky/health/dedup; gaming/streaming policy | ARCHITECTURE §5; tests/bench |
| ROUTE-04 | MRS/MMDB/GeoIP/GeoSite/providers, source/cache/update/rollback; dialer-proxy chains | CONFIGURATION §6; fixtures |
| ROUTE-05 | Live connection metrics/control, preserve active route, simulation/conflicts/hits, optimized ordered indexes | ARCHITECTURE §5; tests/bench |
| TUN-01 | System/RIPS/Mixed/Auto, supported TCP/UDP/ICMP IPv4/v6, CUBIC/BBR3 after validation | SPECIFICATION §4; real Windows |
| TUN-02 | Wintun, routes, independent TUN/DPI/core exclusions, WFP Kill Switch off/proxy-only/strict/persistent | ARCHITECTURE §3,5; driver/leak |
| DPI-01 | Нативный Rust по семантике Zapret2, Lua через mlua, независимый standalone, DIRECT-only, явные режимы/исключения, защита от повторного перехвата и петель | ARCHITECTURE §6; original comparison |
| DPI-02 | Актуальные проверенные стратегии Flowseal на Zapret2, импорт и обновление, original/Ultimate/custom, диагностика, без исполнения BAT/CMD и наследия Zapret1 | ARCHITECTURE §6; fixtures |
| DPI-03 | Include/exclude/auto lists, user/provider sources, provenance/conflicts, atomic update, exclusions first | ARCHITECTURE §6; tests |
| DPI-04 | Quick/standard/extended/custom autoselect, history, manual apply default, four HTTPS probes, configurable tests | ARCHITECTURE §6; integration |
| DPI-05 | Separate network profile, diagnostics, driver/anticheat compatibility without interference, resource updates | ARCHITECTURE §6; real Windows |
| IPC-01 | Clash REST/WS + versioned native API/Named Pipes MessagePack, JSON external, capabilities/events | ARCHITECTURE §7; contracts |
| IPC-02 | Local default, remote off, token/ACL/read/operator/admin/custom, separate privileged auth | ARCHITECTURE §7; security |
| SYS-01 | Rust service AerisCore, headless/portable, watchdog/backoff/safe mode, minimal privilege | ARCHITECTURE §1,8; recovery |
| SYS-02 | aeris from pwsh PATH, no collision with GUI, CLI commands/JSON/help/completion/errors, concurrent GUI/CLI | ARCHITECTURE §1; installed/portable |
| SYS-03 | LocalAppData/portable/ProgramData boundaries, DPAPI, atomic settings/history; no total backup | ARCHITECTURE §7; persistence |
| OBS-01 | Separate logs/metrics, limits/defaults/redaction, nonblocking buffers, history opt-in, scrubbed export | ARCHITECTURE §7, DEFAULTS |
| OBS-02 | Diagnostics manual target/direct/VPN/DNS/leaks, component status, no auto repair | UI-SPECIFICATION §4; tests |
| UI-01 | WinUI3 native MVVM, no network in GUI, two profile-driven modes, fixed widgets, IP/announce policy | UI-SPECIFICATION §1–3 |
| UI-02 | Simple dashboard exact order/cards, separate VPN/DPI controls, profile refresh, full-page navigation | UI-SPECIFICATION §2 |
| UI-03 | Full dashboard six separate cards and exact sidebar/footer layout, no duplicate diagnostics/about nav | UI-SPECIFICATION §3 |
| UI-04 | Proxies list/tab, layouts/cards/sort/icons, groups/selection/pinning/latency/dedup, adaptive columns | UI-SPECIFICATION §4 |
| UI-05 | Profiles add/update/preview/history/rollback/traffic/expiry/HWID, read-only source | UI-SPECIFICATION §4 |
| UI-06 | Connections three tabs, table fields, actions, text-only protocol/outbound colors | UI-SPECIFICATION §4 |
| UI-07 | Routing pages and user rules; DNS overview/settings/cache/queries | UI-SPECIFICATION §4 |
| UI-08 | Zapret2 pages/simple subset, lists/strategies/test results; log tabs/colors | UI-SPECIFICATION §4 |
| UI-09 | Settings, About inside settings, independently customizable colors/Acrylic60%/theme export | UI-SPECIFICATION §1,4 |
| UI-10 | Hover/animations, no visible borders, native fallback, high DPI/accessibility | UI-SPECIFICATION §1 |
| UI-11 | Tray menu exact, hotkeys defaults, one-time disclaimer, persistent selections, startup behavior | UI-SPECIFICATION §5, DEFAULTS |
| UI-12 | RU/EN, notifications categories/defaults, confirmations, offline UI, IPC reconnect | UI-SPECIFICATION §5 |
| UI-13 | Traffic 1s, CPU/RAM 2s, event statuses, suspend rendering, virtualize/cache | UI-SPECIFICATION §6; perf |
| SUB-01 | Remnawave UA/HWID, adapted template, approved headers + legacy, no provider override | PROVIDER-HEADERS; fixtures |
| SUB-02 | Startup/interval/manual update, validate, preserve selection/rules, rollback, live headers | SPECIFICATION §5 |
| SUB-03 | Expiry/quota warning only; 403/404/empty invalid, safe old profile; independent DPI | SPECIFICATION §5; fault tests |
| SEC-01 | No telemetry/MITM/arbitrary privileged execution; redacted logs/DPAPI/least privilege | SPECIFICATION §7; audit |
| SEC-02 | Lua capability/limits, signed trusted resources, path/argument/archive validation, source licenses | ARCHITECTURE §6–8; tests |
| EXT-01 | Слабая связанность GUI/ядра/служб, версионируемые внутренние IPC/контракты, раздельные права и обновления, независимый базовый клиент, без затрат на критическом сетевом пути | ARCHITECTURE §9; архитектурное и системное ревью |
| EXT-02 | Отложено до стабилизации. Универсальные расширения, Plugin Manager/API/SDK, каталоги и пакеты, подписи, разрешения, изоляция, управление жизненным циклом, откат, безопасный режим и инструменты. Конкретные модули отдельно согласуются | ROADMAP; отложенные проверки TESTING |
| REL-01 | 3 repos, independent versions, unique IDs, Inno Setup install/portable/core ZIP, no FlClashZ damage | SPECIFICATION §2 |
| REL-02 | Release/Nightly only, signed manifests, 24h check, changed-only downloads, compatible rollback and switch to Release | RELEASES, SPECIFICATION §8 |
| REL-03 | Exact upstream tracking schedules, reviewed ports, licensed resources, separate updates | UPSTREAM, ROADMAP |
| QA-01 | fmt/Clippy/Roslyn/tests/Audit/Deny/fuzz/security, reviewed stable dependencies, no placeholders | TESTING §1–2, AGENTS |
| QA-02 | Windows 10/11 actual systems, Intel/AMD, install/portable, antivirus/anticheat, driver/network recovery | TESTING §2–3 |
| QA-03 | Same-condition comparative benchmarks, CPU/RAM/latency/throughput/connection/24h/72h/leak | TESTING §4 |
| QA-04 | All release gates, RU/EN docs GitHub Pages/search/channels, examples in CI, public matrix | TESTING §5, ROADMAP |

## Проверенные части требований — 2026-11-27

Указанный результат относится только к конкретным проверенным частям. Статус целого требования остаётся `PARTIAL`; подробности, SHA и пределы проверок приведены в [STATUS.md](STATUS.md).

| ID | Статус | Частичное подтверждение и ограничение |
|---|---|---|
| CFG-01 | PARTIAL | Конфигурационный PR #1 влит в `0bfdafd`. PR #5 на `a311af4` подтвердил 43 теста Windows workspace для проверки полей health-check. PR #6 влита в `48b8dad` (head `3280bdc`); Windows PR CI подтвердил 47 тестов workspace с IPv6, а итоговые main Core CI и dependency policy прошли. Для PR #7 (`6a7e49e`, merge `959dbe2`) Windows Core CI прошёл все 52 теста workspace, итоговые main Core CI и dependency policy также прошли. Фикстура `nameserver-policy.yaml` подтверждает строковую и списочную формы политики из теста сериализации закреплённой сборки FlClashX. Полная схема и совместимость реальных профилей остаются открытыми. |
| DNS-02 | PARTIAL | Тесты `nameserver-policy.yaml` и `fallback-filter.yaml` покрывают формы полей, проверку IPv4/IPv6 CIDR при заданном `dns.fallback`, скрытие значений в ошибках и сохранение рабочего снимка при отказе. Windows CI PR #7 подтвердил 52 теста workspace, итоговые main Core CI и dependency policy прошли. Выбор резолвера, применение политик и фильтров, маршрутизация и проверка утечек остаются открытыми. |
| CFG-02 | PARTIAL | PR #3 на `e5ee837` подтвердил порядок и отключение пользовательских правил и неизменность исходного YAML на уровне снимков. UI, IPC, хранение и действующая маршрутизация остаются открытыми. |
| CFG-03 | PARTIAL | PR #3 подтвердил отказ при конфликте поколений, сохранение правил при обновлении и сохранение рабочего снимка при ошибке. Дисковая история, атомарное применение в сети и активные соединения требуют дополнительных проверок. |
| CORE-03 | PARTIAL | PR #2 влит в `96857f9`. Windows Core CI на `cc7c337` подтвердил IPv4/IPv6 TCP и UDP loopback, ограниченную очередь, отмену и full-duplex TCP. Сетевой lifecycle и нагрузочные сценарии остаются открытыми. |
| SEC-02 | PARTIAL | PR #1 в Aeris-Resources (`fddb5b5`, слита в `855699f`) добавила статическую проверку каталога. Подтверждены 8 тестов, локальная проверка и независимое security review. Подписанных ресурсов и runtime-проверки доверия нет. |
| REL-03 | PARTIAL | Статический каталог проверяет наличие метаданных источника, ревизии и лицензии. Аудит конкретных ресурсов, проверка их совместимости и процедуры обновления остаются открытыми. |
| QA-01 | PARTIAL | Для отдельных ревизий выполнены указанные Rust- и ресурсные проверки. Полный набор QA-01, интеграционные и системные проверки не завершены. |

Do not expand IDs into new requirements not present in the referenced documents. For detailed acceptance criteria retain the full documents and user-approved defaults; test IDs can be finer-grained. Initial status and green branch checks: [STATUS.md](STATUS.md).
