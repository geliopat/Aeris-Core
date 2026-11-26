# Configuration implementation status / Статус конфигурационного модуля

Development branch, not a stable YAML contract.

- The original UTF-8 text is kept privately, unchanged; no serialization back into a provider subscription.
- Single document, 16 MiB input cap, bounded parser budgets; reject duplicate keys, unsupported tags and includes. Diagnostics never print input text, parser excerpts or supplied values.
- Initial structural checks cover basic mode/ports/booleans, TUN and DNS field shapes, proxy/group identities, common references, and an iterative cycle check. Rules preserve provider ordering; user rules are a separate ordered sequence and a preview places them first.
- `aeris config check FILE` returns 0 only for a structural pass, 1 for a read/parse/check error, 2 for an unsupported command. It explicitly reports that runtime compatibility has NOT been verified. No network sockets, DNS, TUN, drivers or activation.
- The unchecked top-level field count only covers root keys. Sections such as `dns`, `tun`, and `proxies` have partial structural checks and can still contain unvalidated nested fields. The unchecked rule matcher count identifies rule kinds outside the checker. Protocol-specific schemas, provider contents, full rule payload validation, sub-rules, dynamic groups, and runtime activation still require implementation.
- For checked rules, the structural checker rejects unrecognized trailing arguments without echoing them. It accepts `no-resolve` and `src` on `IP-CIDR`, `IP-CIDR6`, `GEOIP`, `IP-ASN`, and `RULE-SET`. Both CIDR names accept both address families. The checker validates the address and its prefix length without masking host bits or rewriting the original. These checks cover provider and separate user rules (CFG-01/CFG-02). [Mihomo rule parser](https://github.com/MetaCubeX/mihomo/blob/0455d0d28c4526937380505153e65a843d8612e8/rules/parser.go) · [CIDR constructor](https://github.com/MetaCubeX/mihomo/blob/0455d0d28c4526937380505153e65a843d8612e8/rules/common/ipcidr.go)
- Initial limits and parser choice are implementation decisions, not new user-facing YAML options; adjust against real sanitized FlClashX fixtures and performance/security tests.
- Parser dependencies: serde-saphyr 2.2.0 and serde_json 1.x. Build lockfile and audit dependency licenses/security before merge. No upstream project code copied.

The first fixture is synthetic, not evidence of server or real-profile compatibility.

## Immutable profile candidates

The internal ProfileStore publishes structurally checked immutable snapshots, with independent user rules and a generation precondition. Invalid candidates and stale writers leave the old snapshot unchanged. Existing readers retain the old snapshot; one previous snapshot can be restored with a new generation. This is in-memory candidate storage only; there is no active configuration, disk persistence, live socket migration or runtime rollback yet. User rules with supported simple matchers are checked against candidate proxy/group/provider references. Complex matcher semantics and nested fields remain unchecked. The CLI accepts both `config check` and the documented `config test` alias.

The initial parser CI passed on Windows x64 (45838313971). New snapshot, CLI and concurrency tests require their own successful CI before merge.
