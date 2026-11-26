# Состояние при передаче Codex / Handoff snapshot

Снимок 2026-11-26. Не заменяет текущую проверку HEAD/PR/CI. Требования не равны реализации. При изменениях обновлять SHA и доказательства, не переписывать прошлые результаты.

| Объект | Проверенный SHA | Фактический результат |
|---|---|---|
| Aeris-Core main до документационного коммита | 3e3d0181b36ede8b99cc99ed8525956604804d67 | начальный Rust CLI, Windows CI, upstream report |
| Aeris main | b5ef68f350388356b237b73223dcf4a932c2c5b8 | [CI PASS](https://github.com/geliopat/Aeris/actions/runs/45837557945), WinUI shell; .NET 11/C#15/SDK26100/AppSDK2.7; не полноценный GUI |
| Aeris-Resources main | 935e27360f9a8bd6b22eaf252701c23337cf2827 | [CI PASS](https://github.com/geliopat/Aeris-Resources/actions/runs/45837571789), валидатор и пустой каталог |
| Config PR #1 | 5514bce42bfd32f786f7bf356c79e197667367b0 | [CI PASS](https://github.com/geliopat/Aeris-Core/actions/runs/45841502554), [dependency policy PASS](https://github.com/geliopat/Aeris-Core/actions/runs/45841502611) |
| Transport PR #2 | 1778ab2d3d94cb421d3d5a3537a7f04fe916d8bc | [CI PASS](https://github.com/geliopat/Aeris-Core/actions/runs/45841580149), [dependency policy PASS](https://github.com/geliopat/Aeris-Core/actions/runs/45841580173) |

- [PR #1](https://github.com/geliopat/Aeris-Core/pull/1): draft, feature/configuration-foundation → main. YAML structural parser, source immutability, basic shape/reference/cycle checks, separate user rules, memory snapshots with generation and rollback, CLI check/test, tests. Нет полного schema, runtime activation и реальной совместимости FlClashX.
- [PR #2](https://github.com/geliopat/Aeris-Core/pull/2): draft, feature/transport-foundation → feature/configuration-foundation. Stacked PR; Tokio 1.54.1, explicit-address TCP/UDP, per-operation deadlines, semaphore, cancellation, RAII, loopback tests, committed Cargo.lock. Первоначальный прогон 22 тестов PASS; строгий повторный CI PASS. Нет полноценного VPN, TLS, route binding, DNS, TUN, реальных IPv6/driver tests и bounded caller dispatcher. Обе ветки ещё не слиты в main на момент снимка.
- Cargo Deny теперь PASS на обоих указанных SHA. Прежняя ошибка wildcard локальной зависимости исправлена. Отсутствие известных advisory в проверенном графе не равно доказательству общей безопасности. Проектная лицензия не утверждена; расширенный аудит в ветке PR #1.
- Все начальные Node20 deprecation предупреждения устранены обновлением Actions до Node24; исходные сборки трёх репозиториев прошли. Нет подтверждения работы GUI на реальных Windows 10/11, только компиляция каркаса.
- Нет release automation/signing, Nightly/Release артефактов, драйверных прогонов, TLS/протоколов/DNS/routing/TUN/DPI/API/IPC/service/Remnawave/full UI/installer реализации. Не обещать их наличие.

## Очередность продолжения

1. Проверить текущие HEAD/PR и наличие новых изменений Codex, прочитать веточные документы CONFIG-IMPLEMENTATION, TRANSPORT-IMPLEMENTATION, DEPENDENCY-POLICY.
2. Прочитать документационные уточнения и review. Проверить обязательные checks после изменения base, закрыть выявленные проблемы, согласовать/выполнить слияние #1 по полномочиям, затем retarget #2 к main (проверить diff) и повторно проверить; не дублировать already-merged commits.
3. Завершать config schema/реальные обезличенные fixtures и transport lifecycle/IPv6/backpressure; проектировать TLS и API контракты. План [ROADMAP.md](ROADMAP.md). Не считать этапы закрытыми лишь по первым тестам.
4. Отмечать требования по ID и обновлять статус только по результатам с evidence. Не публиковать stable до полного release gate.
