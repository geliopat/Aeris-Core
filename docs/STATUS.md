# Текущая контрольная точка — 2026-11-27

Изменения и результаты ниже относятся к новым ревизиям относительно исторического снимка далее. Результаты относятся к конкретным проверкам. Статусы требований отдельно учитывают запланированную, отложенную и частично реализованную работу.

## Проверенные изменения

| Репозиторий / изменение | Ревизия | Подтверждённый результат |
|---|---|---|
| Aeris-Core, [PR #1](https://github.com/geliopat/Aeris-Core/pull/1) | merge `0bfdafd`, head `6aa54a9` | [Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45847926292) и [dependency policy](https://github.com/geliopat/Aeris-Core/actions/runs/45847926211) прошли. Локально пройдены 20 Rust-тестов, rustfmt и Clippy. Структурная проверка включает форму CIDR и допустимые параметры известных правил. Полная схема, runtime-применение и межоперационная совместимость остаются открытыми. |
| Aeris-Resources, [PR #1](https://github.com/geliopat/Aeris-Resources/pull/1) | merge `855699f`, head `fddb5b5` | [Resources CI](https://github.com/geliopat/Aeris-Resources/actions/runs/45848776857) прошёл на `main`. Локально и при независимой проверке пройдены 8 тестов статического валидатора. Подписи, происхождение конкретных ресурсов и проверка при обновлении остаются открытыми. |
| Aeris-Core, [PR #2](https://github.com/geliopat/Aeris-Core/pull/2) | merge `96857f9`, head `cc7c337` | [Windows Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45848773347) подтвердил 31 тест workspace, включая IPv6 TCP/UDP loopback. [Проверка зависимостей](https://github.com/geliopat/Aeris-Core/actions/runs/45848773356) прошла. Локально 29 тестов прошли, два IPv6-теста столкнулись с ограничением `::1`, которое независимо воспроизведено штатными сокетами Python. |
| Aeris-Core, [PR #3](https://github.com/geliopat/Aeris-Core/pull/3) | merge `6b8894b`, head `e5ee837` | [Windows Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45890204385) подтвердил все 39 тестов workspace. [Проверка зависимостей](https://github.com/geliopat/Aeris-Core/actions/runs/45890204350) прошла. Покрыты порядок и отключение пользовательских правил, ограничения объёма, сохранение правил при обновлении профиля и конфликт поколений. |
| Aeris-Core, поля health-check групп, [PR #5](https://github.com/geliopat/Aeris-Core/pull/5) | merge `ff1fa1e`, head `a311af4` | [Windows Core CI PR](https://github.com/geliopat/Aeris-Core/actions/runs/45891035355) подтвердил 43 теста workspace на `a311af4`. На `ff1fa1e` прошли [Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45891195223) и [dependency policy](https://github.com/geliopat/Aeris-Core/actions/runs/45891196116). Локально проверены 32 теста конфигурации, снимков и CLI, а также отказ для ошибочного `timeout`. |
| Aeris-Core, [PR #6](https://github.com/geliopat/Aeris-Core/pull/6) | merge `48b8dad`, head `3280bdc` | [Windows Core CI PR](https://github.com/geliopat/Aeris-Core/actions/runs/45931789912) прошёл все 47 тестов workspace, включая IPv6. На итоговом main прошли [Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45931925233) и [dependency policy](https://github.com/geliopat/Aeris-Core/actions/runs/45931925368). |
| Aeris-Core, [PR #7](https://github.com/geliopat/Aeris-Core/pull/7) | merge `959dbe2`, head `6a7e49e` | [Windows Core CI PR](https://github.com/geliopat/Aeris-Core/actions/runs/45932773687) прошёл все 52 теста workspace на `6a7e49e`. На итоговом main прошли [Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45932923267) и [dependency policy](https://github.com/geliopat/Aeris-Core/actions/runs/45932923280). Эти проверки фиксируют состояние сборки и workspace. |
| Aeris-Core, `dns.nameserver-policy` | [синтетический YAML](../crates/aeris-config/tests/fixtures/nameserver-policy.yaml) | На Windows GNU прошли 45 тестов workspace, rustfmt и строгий Clippy. Два IPv6-теста оставлены для Windows CI из-за локального ограничения `::1`. Через `aeris config test` проверены допустимая политика, отказ для неверного списка и скрытие исходных значений в диагностике. Четыре новых теста независимо перепроверены. |
| Aeris-Core, `dns.fallback-filter` | [синтетический YAML](../crates/aeris-config/tests/fixtures/fallback-filter.yaml) | На Windows GNU прошли 50 тестов workspace, rustfmt и строгий Clippy. Два IPv6-теста оставлены для Windows CI из-за локального ограничения `::1`. CLI подтвердил корректную фикстуру и условную проверку CIDR при наличии резервного резолвера. Ошибка возвращает путь поля без исходных значений. |
| Aeris, [PR #1](https://github.com/geliopat/Aeris/pull/1) | merge `5ff8910`, head `bd0a20e` | Зафиксированы ранее согласованные профили Codex и порядок передачи требований. CLI в строгом режиме подтвердил загрузку конфигурации и три дочерних потока. [Client CI](https://github.com/geliopat/Aeris/actions/runs/45849513585) прошёл на `main` для оболочки WinUI 3. |
| Aeris-Core, аудит и `hosts`, [PR #8](https://github.com/geliopat/Aeris-Core/pull/8) | `804bdde` | [Windows Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45968656350) подтвердил Rust 1.89.0, все 61 Rust-тест, включая IPv6, сборку Windows x64 и CLI smoke. [Проверка зависимостей](https://github.com/geliopat/Aeris-Core/actions/runs/45968656168) прошла на том же коде. |
| Aeris, документация и критерии поставки, [PR #2](https://github.com/geliopat/Aeris/pull/2) | `fb231ed` | [Client CI](https://github.com/geliopat/Aeris/actions/runs/45968503801) подтвердил сборку WinUI 3 x64. |
| Aeris-Resources, документация ресурсов, [PR #2](https://github.com/geliopat/Aeris-Resources/pull/2) | `1c8dbd5` | [Resources CI](https://github.com/geliopat/Aeris-Resources/actions/runs/45968504039) подтвердил валидатор и тесты каталога. |
| Aeris-Core, верхнеуровневый `hosts` | [синтетический YAML](../crates/aeris-config/tests/fixtures/hosts.yaml) | Независимый локальный прогон подтвердил 54 из 56 Rust-тестов, rustfmt и строгий Clippy. Два IPv6 loopback теста упёрлись в локальное ограничение `::1`. CLI подтвердил структурную проверку и диагностику без содержимого YAML. DNS-интерпретация остаётся открытой. |

## Статус требований

Только указанные ниже части требований получили новую проверку. Общие ID остаются `PARTIAL`, поскольку обязательные результаты в целом не достигнуты.

| ID | Статус | Основание и оставшееся |
|---|---|---|
| CFG-01 | PARTIAL | Проверены CIDR, поля health-check прокси-групп, явная группа `GLOBAL` и формы `hosts`, `dns.nameserver-policy` и `dns.fallback-filter` из закреплённой сборки FlClashX. Для PR #7 (`6a7e49e`, merge `959dbe2`) прошли Windows Core CI со всеми 52 тестами workspace и итоговые main Core CI/dependency policy. Синтетические примеры подтверждают структуру и сохранение исходного YAML. Полная схема и совместимость реальных профилей остаются открытыми. |
| DNS-02 | PARTIAL | Проверены строковая и списочная формы `hosts` и `dns.nameserver-policy`, типы полей `dns.fallback-filter` и синтаксис IPv4/IPv6 CIDR при наличии резолверов в `dns.fallback`. Для PR #7 прошли Windows Core CI со всеми 52 тестами workspace и итоговые main Core CI/dependency policy. Ошибка оставляет рабочий снимок. Интерпретация `hosts`, выбор резолвера, применение фильтров и маршрутизация в DNS остаются открытыми. |
| CFG-02 | PARTIAL | На уровне in-memory снимков проверены порядок, редактирование и отключение отдельных пользовательских правил при неизменном исходном YAML. Интерфейс, хранение и живое применение ещё в работе. |
| CFG-03 | PARTIAL | При обновлении кандидата сохраняются пользовательские правила, а ошибка или конфликт поколения защищают предыдущий рабочий снимок. Runtime-активация, дисковая история и состояние активных соединений остаются открытыми. |
| CORE-03 | PARTIAL | Проверены IPv4/IPv6 TCP и UDP в loopback, ограниченная очередь ожидания сокетов, отмена, таймауты и full-duplex TCP. Проверки сетевого маршрута, рабочего lifecycle компонентов и нагрузочного поведения остаются открытыми. |
| SEC-02 | PARTIAL | В Aeris-Resources проверен статический валидатор каталога; подписанные ресурсы и runtime-проверка доверия отсутствуют. |
| REL-03 | PARTIAL | Статический валидатор проверяет наличие метаданных источника, ревизии и лицензии. Право распространения и совместимость отдельных ресурсов проверяются отдельно. |
| QA-01 | PARTIAL | Для указанных ревизий пройдены конкретные Rust/ресурсные проверки, включая независимый security review ресурсов; общий перечень QA-01 и тестирование всей системы не выполнены. |

## Поставка

Списки GitHub Releases пусты, отдельных workflow Nightly и Release нет. Перед стабильным выпуском требуется полное выполнение [TESTING.md](TESTING.md) и разрешение владельца. Открыты протоколы, подписки и Remnawave, завершённый GUI и установщик, проверки VPN/DNS/TUN на целевых Windows, безопасность, производительность и 72-часовой прогон.

## Ретроспективный аудит 2026-11-27

Снимок включает объединённые изменения на `main` и локальную ветку текущей проверки `hosts`. На начало проверки открытых PR во всех трёх репозиториях не было. Проверены исходники, тесты, конфигурации, зависимости и лицензии, границы процессов, release-документы и GitHub Actions.

| Область | Базовая ревизия | Что подтверждено и что требуется |
|---|---|---|
| Конфигурация, снимки, CLI и транспорт | Aeris-Core `959dbe2` | [Core CI](https://github.com/geliopat/Aeris-Core/actions/runs/45932923267) и [dependency policy](https://github.com/geliopat/Aeris-Core/actions/runs/45932923280) прошли. Проверяются структурная форма, идемпотентное чтение исходного YAML, обновление снимков, лимиты, таймауты и отмена TCP/UDP. Снимки и транспорт пока библиотечные. CLI проверяет файлы и отклоняет неподдерживаемые команды. Базовая ревизия содержит 52 Rust-теста, включая IPv6 loopback на Windows CI. |
| Интерфейс, права и сборка | Aeris `5ff8910` | Оболочка WinUI 3 собрана на Windows x64. Манифест `asInvoker`, сетевого управления и IPC в текущем клиенте нет. XML и настройки SDK статически проверены. Клиентский CI выполняет restore/build. Тесты UI и запуск на Windows 10 22H2 и Windows 11 x64 ещё требуются. |
| Ресурсы, целостность и лицензии | Aeris-Resources `855699f` | Пустой каталог, нет поставляемых стратегий или драйверов. На Windows с Python 3.12 пройдены все восемь тестов валидатора. Валидатор проверяет объявленные пути и хэши. Перед поставкой нужны проверенные лицензии и подписи, полный учёт файлов сборки и тест Windows junction. |
| Служба, IPC, привилегии | Все три | В исходниках нет службы, Named Pipes, драйверного сетевого пути или загрузчика модулей. Версионирование, ACL и capabilities, отключение/восстановление сетевых объектов, проверка изоляции и работа Kill Switch при сбоях остаются проектными требованиями. Shutdown транспорта отменяет операции. Владельцы должны освободить открытые сокеты. |
| Сборка, поставка и CI | Все три | Сборки и отдельные проверки выше прошли на указанных ревизиях. Nightly/Release, подписи и проверки установки не готовы. Прогоны CI не покрывают весь план TESTING. |

На общем рабочем дереве с исправлениями и `hosts` на Windows GNU прошли rustfmt, строгий Clippy, сборка release и 59 Rust-тестов. Два IPv6-теста локально исключены после независимого воспроизведения ограничения `::1`. Независимое архитектурное, системное, security, QA и итоговое ревью не выявили оставшихся блокирующих замечаний в проверенном объёме. Ревью исправило границы ссылок на `GLOBAL`, ссылку на официальный эталон WireGuard и отсылки к критериям стабильного выпуска. Целевые драйверные Windows и работа новых протоколов остаются вне подтверждённых результатов.

Выявленные расхождения и технический долг:

- Объявленный минимум Rust 1.85 против 1.89 у зафиксированного serde-saphyr. Workspace объявляет 1.89, Windows CI проверяет этот минимум. Подсказка CLI теперь отражает вложенную команду `config <check|test> FILE`. Smoke-проверка обёртки сверяет версию с собранным ядром.
- Структурная проверка ошибочно отклоняла явно объявленную группу `GLOBAL`, допускаемую закреплённым Mihomo. Исправление сохраняет контроль зарезервированных имён, дубликатов и циклов. Семантика `COMPATIBLE`/`PASS-RULE` и допустимость лишних аргументов правил требуют отдельных проверок до объявления совместимости.
- До DNS должен появиться упорядоченный контракт для `nameserver-policy` и тест пересекающихся правил. Текущее дерево для структурной проверки сортирует ключи и сохраняет исходный YAML отдельно. До активации маршрутов нужны проверенный типизированный план и отказ при неподдерживаемом правиле, влияющем на безопасность. До подписок нужен отдельный откат версии провайдера и заголовков с сохранением актуальных пользовательских правил. Текущий откат восстанавливает целый снимок.
- До первой поставки ресурсов ограничить состав пакета проверенными путями каталога. Условия лицензирования трёх проектов и notices остаются открытыми. Закрепление ревизий сторонних GitHub Actions и отключение сохранения учётных данных на checkout учитываются при усилении CI в отдельном проверяемом изменении.
- `PROTO-03` и `DPI-01–05` запланированы на этапы 6 и 10. Совместимость актуальных AmneziaWG и Flowseal на Zapret2 ещё не проверялась. `EXT-01` действует при проектировании. `EXT-02` отложен до стабилизации. Исходники новых эталонов, реализация протоколов и система модулей не входят в текущие изменения.

## Аудит CI/CD 2026-11-27

Исходное состояние перед оптимизацией:

- Aeris-Core `d0c080b`: Windows x64 проверяет минимальный Rust, fmt, Clippy, workspace, release build и CLI. Swatinem кэширует Cargo и результаты в `target`. Политика зависимостей работает отдельным job на Ubuntu.
- Aeris `78b40ab`: Windows x64 восстанавливает и собирает оболочку WinUI 3. Существующий кэш NuGet использует ссылки пакетов из проекта и `global.json`. Lock-файла и тестового проекта нет.
- Aeris-Resources `675172d`: GitHub Actions проверяет пустой каталог и тесты стандартной библиотекой Python. Кэшировать или упаковывать готовые ресурсы пока нечего.

В текущем изменении workflow получили FAST/FULL/CLEAN. Core заменяет кэш `target` ограниченным sccache и отдельным кэшем архивов Cargo. Сборочный Rust закреплён на 1.100.0 по успешному предыдущему Windows CI. Проверка минимума 1.89.0 сохраняется. Клиент закрепляет .NET SDK 11.0.100 и NuGet lock-файл, проверяет архивы, выполняет locked restore и сборку x64. Resources выполняет тот же набор тестов и проверку каталога во всех режимах. Действия закреплены на проверенных SHA, отчёты описывают режим, компоненты и затраченное время. Результат готовой сборки Core/GUI пока не восстанавливается из кэша.

Требования CI-01–06 действуют. Переиспользование через границы репозиториев, независимые служба/драйверы/ресурсные пакеты и сборка дистрибутива требуют соответствующих компонентов. Ускорение оценивается по сопоставимым прогонам. Проверки данных и политики зависимостей на Linux не выпускают клиентские бинарники.

| Репозиторий и проверенный head | FAST | FULL | CLEAN |
|---|---|---|---|
| Aeris-Core `f160383`, [PR #9](https://github.com/geliopat/Aeris-Core/pull/9) | [PASS](https://github.com/geliopat/Aeris-Core/actions/runs/46033215234) | [PASS](https://github.com/geliopat/Aeris-Core/actions/runs/46033250784) | [PASS](https://github.com/geliopat/Aeris-Core/actions/runs/46033255781) |
| Aeris `d313652`, [PR #3](https://github.com/geliopat/Aeris/pull/3) | [PASS](https://github.com/geliopat/Aeris/actions/runs/46033215379) | [PASS](https://github.com/geliopat/Aeris/actions/runs/46033260815) | [PASS](https://github.com/geliopat/Aeris/actions/runs/46033265219) |
| Aeris-Resources `8d88a47`, [PR #3](https://github.com/geliopat/Aeris-Resources/pull/3) | [PASS](https://github.com/geliopat/Aeris-Resources/actions/runs/46033211810) | [PASS](https://github.com/geliopat/Aeris-Resources/actions/runs/46033269690) | [PASS](https://github.com/geliopat/Aeris-Resources/actions/runs/46033274835) |

Core CI подтвердил 61 тест, включая IPv6, проверку минимального Rust и полномасштабную сборку Windows x64 с CLI smoke. [Политика зависимостей](https://github.com/geliopat/Aeris-Core/actions/runs/46033215495) прошла. Клиентский CI подтверждает компиляцию оболочки. Resources подтвердил восемь тестов и пустой каталог. Прошли статическая проверка workflow, QA и независимое ревью; исправлены область контекста runner, экспорт переменных sccache, условия записи, изоляция CLEAN, проверки содержимого NuGet-кэша и раздельная отмена устаревших запусков.

Локальный клиентский FAST на SDK 11.0.100 восстановил зависимости за 14,2 с при промахе и за 4,3 с при попадании. По метаданным NuGet 16 пакетов взяты из проверенного архива объёмом 382 371 103 байта. Четыре SDK-пакета получены из официального источника. Это одна пара локальных замеров без времени передачи кэша GitHub. До интеграции Actions показывал 619 952 438 байт кэшей Core, 955 747 934 байта Aeris и 0 в Resources. Старые записи подлежат штатному вытеснению.

Отдельная проверка YAML `profile` остаётся вне main. Из 51 теста пакета конфигурации и снимков один регрессионный тест обнаружил, что общий декодер превращает `yEs` в boolean до проверки поля, тогда как закреплённый эталон его отклоняет. Исправление должно сохранять исходное написание в целевых полях без регрессий в остальных конфигурациях.

## Актуализация D-26

Решение владельца от 2026-11-27 применено по состоянию на 2026-11-28. Сверены GitHub и изолированные рабочие копии. Исторические checkout и незакоммиченный прототип сохранены.

| Репозиторий | Проверенная исходная ревизия | Состояние |
|---|---|---|
| Aeris-Core | исходный `d0c080b`, CI PR #9 объединён в `e2d2182` | Аудит и проверка `hosts` уже на main. [Windows CI PR #9](https://github.com/geliopat/Aeris-Core/actions/runs/46079127743) и [проверка зависимостей](https://github.com/geliopat/Aeris-Core/actions/runs/46079127751) прошли. |
| Aeris | `46c0287` | Оболочка WinUI 3 и режимы CI. [Client CI на main](https://github.com/geliopat/Aeris/actions/runs/46033667484) прошёл. |
| Aeris-Resources | `11e836b` | Статический валидатор пустого каталога и режимы CI. [Resources CI на main](https://github.com/geliopat/Aeris-Resources/actions/runs/46033535331) прошёл. |

Технические привязки RIPS к порту MIPS, CORE-03 к Tokio, CORE-04 к rustls и DPI-01 к mlua заменены измеряемыми контрактами. SQLite, logging, MVVM и инструменты остаются текущими выборами или кандидатами. MIPS обязателен для сравнения на этапе 9. D-26 сохраняет Windows x64, Rust, WinUI 3, специализированные эталоны, форматы, безопасность, defaults, очередь этапов и прежние критерии выпуска. Конфигурация агентов, модели, reasoning и параллелизм сохранены.

Текущие YAML-валидация, снимки, CLI и TCP/UDP-транспорт сохраняются. На этапе 2 повторно проверяются исходные скаляры YAML, валидация ссылок и циклов, обновление/откат и сохранение пользовательских правил. Приоритет — точная совместимость `profile` без изменения общего декодирования. На этапе 3 требуется подтверждение владения сокетами при остановке, общего дедлайна подключения и гонки готовности I/O с отменой. Работа резолвера и IPC остаётся на последующих этапах.

В существующий реестр добавлены семь источников с 24-часовым наблюдением. Сохранились первоначальные 10 записей, их baseline и интервалы. Начальные наблюдаемые SHA сняты через Git refs. У кандидатов лицензии, изученные SHA и утверждённые реализации ожидают профильной проверки. Монитор разделяет наблюдаемые, исследованные и утверждённые ревизии. Метаданные дают только направления анализа. Новые реализации и лицензионные тексты для переноса ещё не исследовались. Локальный валидатор ссылок, ID и отменённых формулировок, 13 автономных Python-тестов и actionlint прошли. У клиента распарсены 10 TOML-файлов и подтверждено сохранение всех настроек вне developer_instructions. Они не заменяют смысловое ревью.

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
