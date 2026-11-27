# Текущая контрольная точка — 2026-11-27

Изменения и результаты ниже относятся к новым ревизиям относительно исторического снимка далее. Статус требования — только частичный, пока не пройдены все его критерии из документации и интеграционные проверки.

## Проверенные изменения

| Репозиторий / изменение | Ревизия | Подтверждённый результат |
|---|---|---|
| Aeris-Core, [PR #1](https://github.com/geliopat/Aeris-Core/pull/1) | merge `0bfdafd`, head `6aa54a9` | [Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45847926292) и [dependency policy](https://github.com/geliopat/Aeris-Core/actions/runs/45847926211) прошли. Локально пройдены 20 Rust-тестов, rustfmt и Clippy. Структурная проверка включает форму CIDR и допустимые параметры известных правил. Полная схема, runtime-применение и межоперационная совместимость остаются открытыми. |
| Aeris-Resources, [PR #1](https://github.com/geliopat/Aeris-Resources/pull/1) | merge `855699f`, head `fddb5b5` | [Resources CI](https://github.com/geliopat/Aeris-Resources/actions/runs/45848776857) прошёл на `main`. Локально и при независимой проверке пройдены 8 тестов статического валидатора. Подписи, происхождение конкретных ресурсов и проверка при обновлении остаются открытыми. |
| Aeris-Core, [PR #2](https://github.com/geliopat/Aeris-Core/pull/2) | merge `96857f9`, head `cc7c337` | [Windows Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45848773347) подтвердил 31 тест workspace, включая IPv6 TCP/UDP loopback. [Проверка зависимостей](https://github.com/geliopat/Aeris-Core/actions/runs/45848773356) прошла. Локально 29 тестов прошли, два IPv6-теста столкнулись с ограничением `::1`, которое независимо воспроизведено штатными сокетами Python. |
| Aeris-Core, [PR #3](https://github.com/geliopat/Aeris-Core/pull/3) | merge `6b8894b`, head `e5ee837` | [Windows Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45890204385) подтвердил все 39 тестов workspace. [Проверка зависимостей](https://github.com/geliopat/Aeris-Core/actions/runs/45890204350) прошла. Покрыты порядок и отключение пользовательских правил, ограничения объёма, сохранение правил при обновлении профиля и конфликт поколений. |
| Aeris-Core, поля health-check групп, [PR #5](https://github.com/geliopat/Aeris-Core/pull/5) | merge `ff1fa1e`, head `a311af4` | [Windows Core CI PR](https://github.com/geliopat/Aeris-Core/actions/runs/45891035355) подтвердил 43 теста workspace на `a311af4`. На `ff1fa1e` прошли [Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45891195223) и [dependency policy](https://github.com/geliopat/Aeris-Core/actions/runs/45891196116). Локально проверены 32 теста конфигурации, снимков и CLI, а также отказ для ошибочного `timeout`. |
| Aeris-Core, `dns.nameserver-policy` | [синтетический YAML](../crates/aeris-config/tests/fixtures/nameserver-policy.yaml) | На Windows GNU прошли 45 тестов workspace, rustfmt и строгий Clippy. Два IPv6-теста оставлены для Windows CI из-за локального ограничения `::1`. Через `aeris config test` проверены допустимая политика, отказ для неверного списка и скрытие исходных значений в диагностике. Четыре новых теста независимо перепроверены. |
| Aeris, [PR #1](https://github.com/geliopat/Aeris/pull/1) | merge `5ff8910`, head `bd0a20e` | Зафиксированы ранее согласованные профили Codex и порядок передачи требований. CLI в строгом режиме подтвердил загрузку конфигурации и три дочерних потока. [Client CI](https://github.com/geliopat/Aeris/actions/runs/45849513585) прошёл на `main` для оболочки WinUI 3. |

## Статус требований

Только указанные ниже части требований получили новую проверку. Общие ID остаются `PARTIAL`, поскольку обязательные результаты в целом не достигнуты.

| ID | Статус | Основание и оставшееся |
|---|---|---|
| CFG-01 | PARTIAL | Проверены CIDR, поля health-check прокси-групп и формы `dns.nameserver-policy` из закреплённой сборки FlClashX. Синтетические примеры подтверждают структуру и сохранение исходного YAML. Полная схема и совместимость реальных профилей остаются открытыми. |
| DNS-02 | PARTIAL | `dns.nameserver-policy` принимает строку или список строк на селектор. Ошибка формы оставляет рабочий снимок. Выбор резолвера, порядок политик и маршрутизация в работающем DNS остаются открытыми. |
| CFG-02 | PARTIAL | На уровне in-memory снимков проверены порядок, редактирование и отключение отдельных пользовательских правил при неизменном исходном YAML. Интерфейс, хранение и живое применение ещё в работе. |
| CFG-03 | PARTIAL | При обновлении кандидата сохраняются пользовательские правила, а ошибка или конфликт поколения защищают предыдущий рабочий снимок. Runtime-активация, дисковая история и состояние активных соединений остаются открытыми. |
| CORE-03 | PARTIAL | Проверены IPv4/IPv6 TCP и UDP в loopback, ограниченная очередь ожидания сокетов, отмена, таймауты и full-duplex TCP. Проверки сетевого маршрута, рабочего lifecycle компонентов и нагрузочного поведения остаются открытыми. |
| SEC-02 | PARTIAL | В Aeris-Resources проверен статический валидатор каталога; подписанные ресурсы и runtime-проверка доверия отсутствуют. |
| REL-03 | PARTIAL | Статический валидатор проверяет наличие метаданных источника, ревизии и лицензии. Право распространения и совместимость отдельных ресурсов проверяются отдельно. |
| QA-01 | PARTIAL | Для указанных ревизий пройдены конкретные Rust/ресурсные проверки, включая независимый security review ресурсов; общий перечень QA-01 и тестирование всей системы не выполнены. |

## Поставка

Списки GitHub Releases пусты, отдельных workflow Nightly и Release нет. Перед стабильным выпуском требуется полное выполнение [TESTING.md](TESTING.md) и разрешение владельца. Открыты протоколы, подписки и Remnawave, завершённый GUI и установщик, проверки VPN/DNS/TUN на целевых Windows, безопасность, производительность и 72-часовой прогон.

## Следующие проверки

1. Продолжить CFG-01/02/03 по схеме, обезличенным fixtures и обновлению профиля. Подготовить отдельные доказательства совместимости сборки FlClashX, шаблона Remnawave и существующих серверов.
2. Продолжить CORE-03 по сетевому lifecycle и нагрузке. При проектировании TLS и IPC сверять этапы и границы с [ROADMAP.md](ROADMAP.md) и [DECISIONS.md](DECISIONS.md).
3. Обновлять статусы требований по конкретным ревизиям, фикстурам, сценариям и результатам. Проверить все критерии выпуска и получить разрешение владельца перед стабильной публикацией.

---

# Исторический снимок при передаче Codex

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
