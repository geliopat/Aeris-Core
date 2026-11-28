# Эталоны / Upstream references

Начальные зафиксированные ревизии от 2026-11-26, не обязательно последние сегодня. Машиночитаемый источник для мониторинга — [upstream/sources.json](../upstream/sources.json). Лицензии — [UPSTREAM-LICENSES.md](UPSTREAM-LICENSES.md). Для смены утверждённого baseline требуются ревью, тесты и действующее разрешение владельца.

| Репозиторий | Ревизия | Интервал |
|---|---|---|
| [Xray-core](https://github.com/XTLS/Xray-core/tree/b56ffef338c8096c7dbe78eaa76f1cb05a328d6f) | b56ffef338c8096c7dbe78eaa76f1cb05a328d6f | 6h |
| [FlClashX](https://github.com/pluralplay/FlClashX/tree/a9bbb66f6aec3e47359819c93976148a1d270e23) | a9bbb66f6aec3e47359819c93976148a1d270e23 | 24h |
| [MIPS](https://github.com/MetaCubeX/mipstack/tree/9a80a07eaeb9404d5bf416288e989ba7f947c899) | 9a80a07eaeb9404d5bf416288e989ba7f947c899 | 24h |
| [Zapret2](https://github.com/bol-van/zapret2/tree/93fbe9af8110107f44b70dba1ee956a0902f65b6) | 93fbe9af8110107f44b70dba1ee956a0902f65b6 | 6h |
| [Flowseal](https://github.com/Flowseal/zapret-discord-youtube/tree/ffce9179b2b1f2aeb961f4d17274ada3b6d6bea5) | ffce9179b2b1f2aeb961f4d17274ada3b6d6bea5 | 6h |
| [zapret2-gui](https://github.com/mqclass/zapret2-gui/tree/8dd17fcbcca405a50f497a6e2346ca263901d775) | 8dd17fcbcca405a50f497a6e2346ca263901d775 | 24h |
| [uTLS](https://github.com/refraction-networking/utls/tree/360383f893fbe954ff12a701b87295406abbc156) | 360383f893fbe954ff12a701b87295406abbc156 | 24h |
| [rustls](https://github.com/rustls/rustls/tree/e11f98090bee85e6cb2ca03b11819fd145668795) | e11f98090bee85e6cb2ca03b11819fd145668795 | 24h |
| [Wintun](https://github.com/WireGuard/wintun/tree/ec0a6b98456fe1ba52567bb2add4bbf5f64315a1) | ec0a6b98456fe1ba52567bb2add4bbf5f64315a1 | 24h |
| [WinDivert](https://github.com/basil00/WinDivert/tree/4f311999b0d93b88f09450bb56ba9cc8cb028d9c) | 4f311999b0d93b88f09450bb56ba9cc8cb028d9c | 24h |

Mihomo v1.19.34, resolved commit 0455d0d28c4526937380505153e65a843d8612e8, локально заменённый sing-tun v0.4.29 и MIPS pin — проверять через [FlClashX/core/go.mod](https://github.com/pluralplay/FlClashX/blob/a9bbb66f6aec3e47359819c93976148a1d270e23/core/go.mod). Не заменять эталон сборки на vanilla Mihomo. Точный текущий commit каждого транзитивного модуля проверять по исходному go.mod/go.sum/replace.

## Кандидаты на исследование

Сборка Mihomo из FlClashX остаётся обязательным эталоном поддерживаемого YAML и сетевого поведения. Оригинальный Xray определяет совместимость VLESS, REALITY, Vision, XHTTP и XUDP. Дополнительные источники работают в пределах своих ролей. Проверенные внутренние решения разрешено адаптировать по [D-26](DECISIONS.md#d-26--выбор-реализаций).

| Оригинальный репозиторий | Ветка | Копия владельца | Роль при достижении этапа |
|---|---|---|---|
| [MetaCubeX/mihomo](https://github.com/MetaCubeX/mihomo) | `main` | — | Дополнительный источник исправлений, форматов, алгоритмов и реализаций на этапах 2–9 |
| [chen08209/Clash.Meta](https://github.com/chen08209/Clash.Meta) | `FlClash` | [geliopat/Clash.Meta](https://github.com/geliopat/Clash.Meta) | Модификации ядра для FlClash на этапах 2–9 |
| [Vialibrex/FlClash](https://github.com/Vialibrex/FlClash) | `main` | [geliopat/FlClash](https://github.com/geliopat/FlClash) | Go-обвязка, клиентская и Windows-интеграция, этапы 12–15 и сопряжённые сетевые механизмы |
| [ibigbug/clash-rs](https://github.com/ibigbug/clash-rs) | `master` | [geliopat/clash-rs](https://github.com/geliopat/clash-rs) | Rust-архитектура, конфигурация, транспорт, протоколы, DNS, маршруты и TUN на этапах 2–9 |
| [YuanDevTeam/Vole](https://github.com/YuanDevTeam/Vole) | `main` | [geliopat/Vole](https://github.com/geliopat/Vole) | Встраивание, транспорт, протоколы, DNS, стеки и оптимизации на этапах 3–7 и 9 |
| [meow-rs/meow-rs](https://github.com/meow-rs/meow-rs) | `main` | [geliopat/meow-rs](https://github.com/geliopat/meow-rs) | Конфигурация, сетевые механизмы, маршрутизация и сравнение стеков на этапах 2–9 |
| [cfal/shoes](https://github.com/cfal/shoes) | `master` | [geliopat/shoes](https://github.com/geliopat/shoes) | Транспорт и протокольные реализации на этапах 3–6, применимые компоненты стека на этапе 9 |

Перед сравнением на профильном этапе подтвердить связь с upstream, отслеживаемую ветку, точную ревизию, исходники нужного компонента, лицензии, зависимости и тесты. Один проект может служить источником архитектуры, алгоритмов, исправлений, оптимизаций, диагностики, тестов или ресурсов. Отдельно фиксировать роль протокольного или функционального эталона, кандидата на портирование, Rust-зависимость либо эксперимент. Чистые ветки зеркал сохранять. Дополнительные подходящие проекты разрешено искать по тем же критериям.

### Модифицированный FlClash

Для релиза FlClash брать SHA ядра из фактического Git-подмодуля. `FlClash` и `main` у Clash.Meta обозначают разные ветки. По профильным этапам сравнивать точные ревизии официального Mihomo, сборки FlClashX, модифицированного FlClash и его Go-обвязки с текущим Aeris и альтернативными Rust-реализациями. Изучать полный релевантный diff, историю, зависимости, тесты и наблюдаемое поведение. Проверять парсинг YAML и конкурентное применение профилей, providers/GeoData, DNS-события и кэш, время и причины завершения соединений, симуляцию маршрутов и цепочки, селекторы/health checks, очереди и IPC, TUN/прокси/маршруты Windows и восстановление после сна. Приоритет у проверенного решения, соответствующего контракту Aeris.

### RIPS на этапе 9

В сравнении участвуют MIPS, сетевые механизмы Mihomo и FlClash, clash-rs, Vole, meow-rs, применимые компоненты shoes, smoltcp и другие подходящие нативные стеки. Допускается исследование архитектурных идей gVisor при разработке самостоятельного Rust-компонента. Сравнивать обработку TCP/UDP/ICMP, IPv4/IPv6, таймеры, очереди, память, congestion control, lifecycle и интеграцию с Windows. За кандидатом закрепляются доказательства и точный SHA после профильных проверок.

## DPI и AmneziaWG

На этапе 10 проверить текущие ревизии всех трёх проектов в утверждённом порядке.

1. [Flowseal на Zapret2](https://github.com/Flowseal/zapret-discord-youtube) — основной технический, архитектурный и функциональный эталон после завершения перехода.
2. [Оригинальный Zapret2](https://github.com/bol-van/zapret2) — семантика механизмов и пакетов, параметры движка и Lua API.
3. [Zapret2-gui](https://github.com/mqclass/zapret2-gui) — совместимые стратегии, функции, автоматизация и проектные решения. Доступна [копия владельца](https://github.com/geliopat/zapret2-gui).

По уточнению владельца, переход Flowseal на winws2 начался 2026-11-26. Если на этапе 10 он ещё не завершён, зафиксировать фактическое состояние без присвоения статуса полноценного эталона. Архитектура использует только семантику Zapret2. Ревизии выше в таблице относятся к исходному мониторингу. Ревизии реализации будут выбраны по проверкам на этапе 10.

На этапе 6 сверить обычный WireGuard с [официальной спецификацией и реализациями WireGuard](https://www.wireguard.com/protocol/). Определить актуальные и предыдущие официальные варианты клиентского AmneziaWG, сверить их параметры и провести межоперационные проверки. Эталоны: [amneziawg-go](https://github.com/amnezia-vpn/amneziawg-go), [amneziawg-tools](https://github.com/amnezia-vpn/amneziawg-tools), [amneziawg-windows](https://github.com/amnezia-vpn/amneziawg-windows), [amneziawg-linux-kernel-module](https://github.com/amnezia-vpn/amneziawg-linux-kernel-module) и [amnezia-client](https://github.com/amnezia-vpn/amnezia-client). Официальные исходники определяют протокол независимо от охвата Mihomo. Ревизии реализации выбираются и фиксируются на этапе 6.

Текущее обновление требований не открывает исследование этих исходников или обновление upstream. Для переноса функций, публикации ресурсов и обновлений применяются проверки лицензий, безопасности и совместимости.

## UI и заголовки

Только чтение: [geliopat/FlClashZ, fix/zapret2-stability](https://github.com/geliopat/FlClashZ/tree/b279c9a7e6f37643ac23214ae180fb58ae41999d) (HEAD проверен при передаче, не означает что все файлы повторно прочитаны именно на этом SHA). Ранее изученные пути: lib/views/proxies/{list,tab,group_tabs,card,common,setting,proxies}.dart; lib/views/dashboard/provider_mode.dart и dashboard/widgets; lib/models/provider_headers.dart; lib/providers/zapret_diagnostics.dart. Найти актуальные пути при изменении структуры. FlClashZ — визуальный/функциональный эталон страниц, но поздние макеты Aeris приоритетнее. Не изменять FlClashZ.

Remnawave: точную ревизию API/формата HWID и response rules закрепить при реализации подписок по реальным документации и тестовым fixtures. Не переносить личные URL, токены, IP и реальные ключи в публичные fixtures.

## Правила обновления

XHTTP и DPI сохраняют шестичасовой приоритет. Новые кандидаты наблюдаются существующим monitor workflow каждые 24 часа. Исходные интервалы и закреплённые ревизии сохранены. Отчёт отдельно показывает наблюдаемый HEAD, исследованный SHA и baseline или утверждённый SHA реализации. Для регистрации достаточно проверенной ветки. Начальные SHA новых веток записаны по Git refs от 2026-11-28 в `observed_sha`, без переноса в `researched_sha` или `approved_implementation_sha`. Каждый текущий HEAD остаётся в отчёте запуска. Метаданные заголовков коммитов дают направления анализа, а доказанное изменение функций, безопасности, протокола, совместимости, Windows, производительности, архитектуры или зависимостей требует профильного ревью. При переносе фиксировать diff, лицензию, происхождение, совместимость и тесты. Перед сменой baseline получить разрешение владельца. Nightly возможен после действующих проверок. Не сливать автоматически чужой код или считать Rust-порт идентичным без тестов. Подписанные ресурсы допускают отдельное автоматическое обновление после проверки совместимости и прав распространения.
