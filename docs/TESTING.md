# Aeris — тестирование и критерии выпуска

Обязательные проверки по мере появления компонентов. Фактически выполненные прогоны и их пределы см. в [STATUS.md](STATUS.md). Компиляция подтверждает сборку.

## 1. CI

| Проверка | Триггер |
|---|---|
| rustfmt, Clippy, Roslyn Analyzers, dotnet format, unit | Каждый коммит |
| Integration/contract tests | Каждый PR |
| YAML/Mihomo compatibility | Изменения ядра/формата |
| XHTTP/Xray interop | Изменения транспорта/обновление эталона |
| Zapret2/Lua compatibility | DPI и ресурсы |
| API/IPC compatibility | Ядро или клиент |
| Cargo Audit/Deny, dependency security/licences | Ежедневно и перед релизом |
| Fuzz parsers/network/Lua | Регулярно и перед релизом |
| Bench, memory/handle leaks, DNS/IP leaks, install/update/remove | Перед Release |

Кэш Rust/NuGet/build, отдельные workflow ядра/GUI, отмена устаревших checks одной ветки, отчёты ошибок, запрет merge при fail обязательных checks. Nightly только при изменениях и успешных проверках. Windows 11 x64 и 10 22H2 x64, Intel/AMD/разные ISA, install/portable/standalone, реальные Windows машины для TUN/WFP/WinDivert/Wintun, антивирусы/античиты без вмешательства в них. Не подменять реальные драйверные прогоны стандартным runner.

## 2. Функциональность

- Парсер: поддерживаемый YAML, ошибочные поля, конфликтующие имена, циклы групп/маршрутов, invalid refs, large inputs, атомарный reload/rollback. Исходная подписка read-only; пользовательские правила first-match до провайдерских; отсутствие `nooverride`.
- Протоколы: каждый утверждённый transport/auth/TLS variant против эталонного сервера, TCP/UDP, IPv4/IPv6, XHTTP auto/packet-up/stream-up/stream-one, XMUX и separate paths/settings, XUDP, REALITY/Vision. Проверка старых и новых версий и реальных fixtures.
- DNS: UDP/TCP/DoH 1/2/3/DoT/DoQ, DNSSEC если включён, cache TTL/negative/prefetch/stale/coalescing, Fake-IP v4/v6, hybrid, policies/hosts, bootstrap through proxy, network change, hijack, DNS Always On/DNS Proxy Access; независимые очистки, никаких утечек в незаявленный resolver. ECH/шифрованные DNS — проверять ограничения, не обещать полное обнаружение.
- Routing: все типы matcher, order, groups/select/health/fallback/smart/adaptive, sticky, dedup, GeoData/provider validation & rollback, chains, active route preservation, user rule priority, exclusions, explicit ZAPRET/DIRECT-NO-ZAPRET; Kill Switch fail-closed при заданной политике.
- TUN: System/RIPS/Mixed/Auto, IPv4/IPv6/TCP/UDP/ICMP, route/driver handling, sleep/network change, bypass, повторный перехват и конфликты. Не считать функцию реализованной, если опция только парсится.
- DPI на этапе 10. Проверить Lua API, семантику пакетов и параметры против Zapret2, актуальные стратегии Flowseal на Zapret2, импорт, обновление и ошибки несовместимости, отсутствие исполнения BAT/CMD, include/exclude/auto, provenance, конфликты, автоподбор/историю, четыре HTTPS-проверки и опциональные отдельные UDP/QUIC проверки. Проверить WinDivert и standalone. На этапе 11 проверить защиту от повторного перехвата и петель, исключение VPN-транспорта и независимые жизненные циклы DNS/VPN/DPI.
- WireGuard/AmneziaWG на этапе 6. Проверить обычный WireGuard и каждый актуальный и предыдущий официальный вариант AmneziaWG против официальных серверов, включая параметры обфускации, несовместимые комбинации и распознавание форматов. Проверить совместимые конфигурации Mihomo/FlClashX/FlClashZ и адаптированный Remnawave по мере готовности соответствующих компонентов. Проверки общих маршрутов, DNS, TUN, групп и цепочек выполняются на этапах их интеграции.
- CLI/IPC: `aeris` доступен из любой директории pwsh в каждой поставке, разрешение имени при установленном GUI, JSON и exit codes, concurrent GUI/CLI, version/capability negotiation, ACL/roles/token, запрет arbitrary privileged commands, reconnect, offline module controls.
- GUI: 2 режима строго по providermode, IP и фиксированные widgets, simple announce/no title, full no announce, sidebar layout/footer, profile refresh without navigation, переходы на полные страницы вместо popup, independent toggles, 4 DPI pings, DPI scaling, theme/fallback, animation/accessibility, RU/EN, tray, hotkeys default off, offline operation, notification dedup, safe confirmations.
- Подписки: UA/HWID, отдельный Remnawave template, headers/legacy precedence, URLs/base64, malformed/403/404/empty, restore last working config, no local auto cutoff on expiry/quota, preserve selection/user rules. Проверить обновление без разрыва там, где заявлено.
- Update: trusted signing keys, tamper/hash/signature/rollback/compatibility, invalid archives/path traversal, interrupted update, driver confirmation, install/portable/core, uninstall preserves чужие объекты, PATH clean.

## 3. Устойчивость

Сценарии: потеря сети, смена Wi-Fi/Ethernet, сон/гибернация, недоступность узла/резолвера, неправильная стратегия, авария отдельного модуля, авария всего core и GUI, потеря IPC, некорректный YAML, обновление с ошибкой, переполнение очередей, недостаток RAM, повреждение ресурсов, аварийное выключение. Проверять ожидаемое состояние и время восстановления, отсутствие restart loops, backoff/safe mode, сохранение причины, cleanup собственных маршрутов/фильтров без нарушения Kill Switch. При падении целого core честно учитывать недоступность всех внутренних модулей. Не считать выключенные пользователем компоненты аварией.

## 4. Бенчмарки

| Объект | Эталон |
|---|---|
| XHTTP | Xray-core и Mihomo FlClashX |
| Vision | Xray-core |
| Остальные протоколы | Mihomo FlClashX (и оригиналы для межоперационности) |
| RIPS/System | MIPS/System Mihomo |
| DNS/routing | Mihomo |
| DPI | Flowseal на Zapret2 и оригинальный Zapret2 в сопоставимых режимах |
| GUI | FlClashZ |

Метрики: RAM idle/load, CPU при равной пропускной способности, max TCP/UDP throughput, RTT/jitter, DNS latency/throughput/cache, большие rulesets, concurrent connections, startup/restart, 24h и 72h endurance, handles/memory leaks. Одинаковые OS/hardware/network/server/config, фиксировать версии/условия, повторения и разброс, не сравнивать несопоставимые режимы. Публиковать методику и результаты; существенная регрессия блокирует релиз до расследования. Численные пороги установить на измеренной baseline, не придумывать цели из неподтверждённых цифр. Rust сам по себе не гарантирует ускорение.

## 5. Release gate 1.0.0

- [ ] Все обязательные протоколы, XHTTP/XUDP/REALITY/Vision и TUN System/RIPS/Mixed/Auto проверены.
- [ ] DNS, Fake-IP, routing/groups/providers, Rust DPI и поддерживаемые Lua стратегии работают; VPN/DPI независимы.
- [ ] Remnawave адаптированный template, утверждённые headers и HWID проверены.
- [ ] CLI standalone, REST/WS и IPC/служба проверены.
- [ ] Простой и полный WinUI3, RU/EN, install/portable/core packages готовы.
- [ ] Независимое signed update/rollback, drivers и миграции проверены.
- [ ] Документация, матрица совместимости и лицензии актуальны.
- [ ] Security, DNS/IP leaks, реальные Windows тесты, сравнение производительности пройдены.
- [ ] 72h стабильности подтверждены; критических известных ошибок нет.
- [ ] Нет заглушек или неподтверждённых обещаний совместимости.

Все пункты обязательны. Статус checklist — не пройден до получения результатов реальных проверок.

## 6. Отложенные проверки модулей

После стабилизации проверить версионную совместимость пакетов и зависимостей, подписи и целостность, разрешения и отказ при запрещённой операции, изоляцию стороннего кода от привилегированных процессов, сбои изолированных модулей, откат и безопасный запуск. Проверить работу клиента и ядра без модулей, независимые обновления и отсутствие регрессий сетевого тракта.

## 7. CI/CD, кэширование и измерения

- Запустить FAST холодным и повторно на одинаковых релевантных входах. Подтвердить одинаковый объём обязательных проверок. Проверить отсутствие второго большого кэша компиляции и совместимость инструментов.
- Изменить один компонент и отдельно его зависимость или применимый контракт. Сверить точную инвалидацию и переиспользование независимых результатов. При передаче между jobs или репозиториями проверить SHA источника, версию, издателя, результат workflow и digest. Проверить отказ для повреждённого, просроченного или несовместимого артефакта.
- Запустить FULL при наличии готового результата. Подтвердить повторную сборку собственных компонентов. Запустить CLEAN и подтвердить отсутствие restore/save, sccache и предыдущих результатов. Перепроверить обязательные тесты для обоих режимов.
- Проверить границы доверия на PR и доверенной ветке, размеры путей кэширования, причины пропусков или вытеснения и отсутствие приватных данных. В CLEAN проверить закреплённые зависимости, не запускать публикацию и ключи подписи в тестах.
- Summary содержит режим, попадания и промахи, компоненты и причины пересборки, версии инструментов, статистику sccache, подготовку, компиляцию, тестирование, упаковку и итоговое время. Для отсутствующего этапа указать статус. Сравнивать холодные, повторные и частичные сборки на одном типе Windows runner при сопоставимых условиях. Фиксировать разброс и объём кэша. Подтверждать ускорение только по измерениям.
