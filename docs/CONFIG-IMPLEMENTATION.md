# Configuration implementation status / Статус конфигурационного модуля

Development branch, not a stable YAML contract.

- The original UTF-8 text is kept privately, unchanged; no serialization back into a provider subscription.
- Single document, 16 MiB input cap, bounded parser budgets; reject duplicate keys, unsupported tags and includes. Diagnostics never print input text, parser excerpts or supplied values.
- Initial structural checks cover basic mode/ports/booleans, TUN and DNS field shapes, proxy/group identities, common references, and an iterative cycle check. Rules preserve provider ordering; user rules are a separate ordered sequence and a preview places them first.
- `aeris config check FILE` returns 0 only for a structural pass, 1 for a read/parse/check error, 2 for an unsupported command. It explicitly reports that runtime compatibility has NOT been verified. No network sockets, DNS, TUN, drivers or activation.
- Only a small subset of fields is inspected; unchecked fields and rule matchers are reported. Unknown nested fields, protocol-specific schemas, numeric/URI edge cases, provider contents, rule matcher semantics, sub-rules, dynamic groups, user-rule semantic checks and atomic activation still require implementation. Do not infer runtime support from this check.
- Initial limits and parser choice are implementation decisions, not new user-facing YAML options; adjust against real sanitized FlClashX fixtures and performance/security tests.
- Parser dependencies: serde-saphyr 2.2.0 and serde_json 1.x. Build lockfile and audit dependency licenses/security before merge. No upstream project code copied.

The first fixture is synthetic, not evidence of server or real-profile compatibility.
