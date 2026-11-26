# Эталоны / Upstream references

Начальные зафиксированные ревизии от 2026-11-26, не обязательно последние сегодня. Машиночитаемый источник для мониторинга — [upstream/sources.json](../upstream/sources.json). Лицензии — [UPSTREAM-LICENSES.md](UPSTREAM-LICENSES.md). Смена baseline — после review и тестов, а не автоматически.

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

## UI и заголовки

Только чтение: [geliopat/FlClashZ, fix/zapret2-stability](https://github.com/geliopat/FlClashZ/tree/b279c9a7e6f37643ac23214ae180fb58ae41999d) (HEAD проверен при передаче, не означает что все файлы повторно прочитаны именно на этом SHA). Ранее изученные пути: lib/views/proxies/{list,tab,group_tabs,card,common,setting,proxies}.dart; lib/views/dashboard/provider_mode.dart и dashboard/widgets; lib/models/provider_headers.dart; lib/providers/zapret_diagnostics.dart. Найти актуальные пути при изменении структуры. FlClashZ — визуальный/функциональный эталон страниц, но поздние макеты Aeris приоритетнее. Не изменять FlClashZ.

Remnawave: точную ревизию API/формата HWID и response rules закрепить при реализации подписок по реальным документации и тестовым fixtures. Zetklif: эталон конвертера, точный репозиторий/ревизия требуют проверки. Не выдумывать ссылки. Не переносить личные URL, токены, IP и реальные ключи в публичные fixtures.

## Правила обновления

XHTTP и DPI приоритетны. Начальный мониторинг только отчёт об изменении head. Для переноса: diff, лицензия, provenance, анализ совместимости, целевые тесты, review, обновление baseline, Nightly после gate. Не сливать автоматически чужой код или считать Rust-порт идентичным без тестов. Подписанные ресурсы допускают отдельное автоматическое обновление после проверки совместимости и прав распространения.
