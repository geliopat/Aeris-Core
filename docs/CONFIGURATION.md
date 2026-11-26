# Aeris Core — конфигурация

Проект контракта. Mihomo YAML — основа, но параметры новых модулей требуют формального schema и проверки реализации. Не считать приведённые примеры готовой рабочей конфигурацией. Имена расширений, не закреплённые явно, согласовать на этапе реализации.

## 1. Форматы и применение

- Mihomo YAML из сборки FlClashX и поддерживаемый Clash YAML; собственные расширения Aeris. Remnawave использует отдельный адаптированный шаблон и правило ответа. URI/QR для поддерживаемых ссылок. Не конвертировать полные конфиги Xray/sing-box.
- Исходный YAML подписки read-only. Нет произвольного override, редактирования оригинала или Lua-скриптов для замены параметров подписки. `nooverride` отсутствует. Пользовательские правила — отдельная структура, сохраняемая при обновлении.
- Приоритет: пользовательские правила в их порядке → правила провайдера в исходном порядке → завершающий MATCH провайдера (если он присутствует и расположен корректно). Не переставлять исходные правила молча; предупреждать о недостижимых правилах. Первое совпадение. Пользовательские правила могут быть отключены.
- Runtime-выбор прокси, состояния компонентов и разрешённые настройки GUI не являются редактированием YAML. Локальные самостоятельные конфиги для CLI поддерживаются.
- Парсинг, проверка типов, ссылок, циклов, совместимости и безопасности до активации; ошибки не игнорировать. Неподдерживаемый параметр не объявлять работающим. Атомарная активация, rollback, сохранить активные соединения там, где безопасно. Подписки: история, просмотр, обновление, откат, сохранение выбора.

## 2. Совместимый синтаксис

Базовые разделы: `proxies`, `proxy-groups`, `proxy-providers`, `rules`, `rule-providers`, `dns`, `hosts`, `tun`, `sniffer`, `mode`, `external-controller`, `external-ui`, `listeners`, `allow-lan`, `bind-address`, `lan-allowed-ips`, `lan-disallowed-ips`, `global-ua`, `find-process-mode`, `profile`, `geodata-mode`, `geo-auto-update`, `geox-url`, `ipv6`, `log-level`, параметры локальных портов и secret при поддержке. SMUX и ASN/sub-rules — в утверждённом подмножестве. Не подразумевать автоматически поддержку каждого ключа всех версий Mihomo; проверять по COMPATIBILITY.

Иллюстрация базового синтаксиса и утверждённого имени стека (не полный production-конфиг):

```yaml
mode: rule
ipv6: true
tun:
  enable: true
  stack: rips
  auto-route: true
  strict-route: true
  dns-hijack:
    - any:53
    - tcp://any:53
dns:
  enable: true
  enhanced-mode: fake-ip
  respect-rules: true
  nameserver:
    - https://resolver.example/dns-query
proxy-groups:
  - name: Example
    type: select
    proxies:
      - DIRECT
rules:
  - MATCH,Example
```

Пример использует зарезервированный домен `.example`, не действующий резолвер. Политики Aeris-расширений, IPv6 и DNS должны пройти тесты; значения по умолчанию не угадывать по этому примеру.

## 3. Протоколы

VLESS с REALITY, Vision, XHTTP, XUDP; VMess, Trojan, SS/2022, Hysteria2, TUIC, AnyTLS, WireGuard/AmneziaWG 2.0, SOCKS5, HTTP/HTTPS, NaiveProxy (HTTP/2 и HTTP/3 по возможностям эталона). Проверить формат `network`, `tls`, `reality-opts`, `client-fingerprint`, `flow`, `xhttp-opts`, `dialer-proxy` по реальному эталону FlClashX и Xray-core. XHTTP modes auto/packet-up/stream-up/stream-one, XMUX, separate download settings и пути — не вводить несовместимый формат без документации миграции. Неподдерживаемые протоколы перечислены в SPECIFICATION.

## 4. TUN и транспорт

System/RIPS/Mixed/Auto, Mixed TCP System + UDP RIPS, RIPS default после проверки. Системный proxy при разрешении. IPv4/IPv6, исключения локальных адресов и маршрутов, приложений/процессов/интерфейсов по технической возможности и отдельно для TUN/DPI/ядра. Настраиваемые MTU, таймауты, параметры стека и congestion control CUBIC/BBR3 там, где реально реализовано. Не дублировать эти параметры HTTP-заголовками. Точные новые поля: TBD.

## 5. DNS

Поддержать `hosts`, `nameserver`, `default-nameserver`, `proxy-server-nameserver`, `direct-nameserver`, `nameserver-policy`, `fallback`, `fallback-filter`, `respect-rules`, `enhanced-mode`, `fake-ip-range`, `fake-ip-filter`, `store-fake-ip` и связанные параметры при проверке совместимости. Поддержать маршрутизацию DNS `https://resolver.example/dns-query#Имя группы`; bootstrap не должен зациклиться. Реальные URI резолверов в профилях пользователя, не вшивать личный сервер разработчика в дистрибутив.

Новые возможности требуют точной схемы: группы DNS (select/parallel/sequential/fallback/load-balance/smart), UDP/TCP/DoH h1/h2/h3/DoT/DoQ, DNSSEC optional, Hybrid, IPv6 Fake-IP, negative caching/prefetch/serve-stale/coalescing/TTL, раздельная персистентность, DNS Always On, DNS Proxy Access, DNS Kill Switch, отдельная политика DPI DNS, standard/strict/custom/off hijack, запрет известных шифрованных DNS endpoints optional, pin/ECS/stamps/rebinding exceptions, Happy Eyeballs/NAT64/DNS64. По умолчанию system DNS только при явной настройке, standard hijack с fail-closed согласно политике, fallback только указанный. Оптимизацию кэша выбрать по результатам бенчмарков, не закреплять W-TinyLFU без проверки.

## 6. Маршрутизация

Правила по domain/IP/ASN/process/port/protocol/interface и логические комбинации. Поддержать согласованное подмножество Mihomo: DOMAIN, DOMAIN-SUFFIX, DOMAIN-KEYWORD, IP-CIDR, IP-CIDR6, GEOIP, GEOSITE, IP-ASN, PROCESS-NAME, PROCESS-PATH, RULE-SET, AND/OR/NOT, MATCH, sub-rules и другие используемые проверенные типы. Синтаксис новых matcher согласовать. DIRECT, REJECT, REJECT-DROP, PASS согласно реальной семантике эталона; маршрут через PROXY/именованный исходящий узел или группу. PROXY как литеральное зарезервированное имя не вводить без проверки эталона/согласования; явные Aeris-маршруты ZAPRET и DIRECT-NO-ZAPRET. ZAPRET при выключенном DPI — DIRECT согласно согласованной политике, кроме запретов Kill Switch/REJECT. DPI не изменяет proxy-транспорт. `dialer-proxy` для цепочек.

Группы select/url-test/fallback/load-balance/smart/adaptive. Тест URL/interval/lazy, дедуп проверок, реальные измерения, sticky/weights/least latency/least connections. Не объявлять данные об удалённой нагрузке без источника. Геоданные и rule providers MRS/MMDB/YAML/text в поддерживаемых вариантах; update/rollback/validation. Прокси-группы с hidden, description, icon, порядок и выбранные элементы как в поддерживаемом формате. Kill Switch off/proxy-only(default)/strict/persistent; exact schema TBD.

## 7. Zapret2 и клиентские настройки

Параметры самого DPI — в конфигурации ядра, точная схема TBD; не дублировать HTTP-заголовками. Режимы DIRECT-only/list-based, STRICT SPLIT/FULL BYPASS для совместимых стратегий, standalone/custom, исключения. STRICT SPLIT ограничивает обработку подходящим трафиком согласованных списков (YouTube/Discord/Google для соответствующего сценария); FULL BYPASS применяет совместимую стратегию на её перехватываемых портах, но не отменяет границу VPN-транспорта, DIRECT-NO-ZAPRET, REJECT и Kill Switch. Ultimate/custom могут определять собственные условия; UI явно показывает ограничения режима, отдельная DNS-политика, параметры таймаутов/портов, указание стратегий и списков с источниками. Не запускать произвольные BAT/CMD. Актуальные ресурсы могут обновляться отдельно после проверки.

Настройки GUI (тема, анимации, горячие клавиши, локализация, уведомления, автозапуск) отдельно от YAML, интерфейсный режим исключительно из профиля. Пользователь не редактирует подписку. Нет глобального экспорта/импорта всех настроек; внутренние rollback snapshots не являются пользовательской функцией бэкапа.

## 8. До объявления формата стабильным

Создать машиночитаемую схему, набор валидных/невалидных fixtures, документацию всех значений/defaults, карту поддерживаемых параметров с источником и версией. Каждое расширение покрывается parser/runtime/reload тестами. Не выдавать предполагаемые имена параметров за утверждённый рабочий синтаксис. 
