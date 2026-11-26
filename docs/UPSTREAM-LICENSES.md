# Upstream inventory and preliminary license audit

Checked repository metadata and reference commits on 2026-11-26. These are review baselines, not statements of compatibility or a license approval. No upstream source or binary is copied into this initialization.

| Source | Ref | GitHub license metadata | Purpose |
|---|---|---|---|
| [XTLS/Xray-core](https://github.com/XTLS/Xray-core) | main | MPL-2.0 | XHTTP, XUDP, REALITY, Vision |
| [pluralplay/FlClashX](https://github.com/pluralplay/FlClashX) | main | GPL-3.0 | Authoritative Mihomo assembly / UI reference |
| [MetaCubeX/mipstack](https://github.com/MetaCubeX/mipstack) | master | MPL-2.0 | RIPS reference; assembly pin 9a80a07eaeb9404d5bf416288e989ba7f947c899 |
| [bol-van/zapret2](https://github.com/bol-van/zapret2) | master | unverified | DPI and Lua semantics |
| [Flowseal/zapret-discord-youtube](https://github.com/Flowseal/zapret-discord-youtube) | main | NOASSERTION | Strategy/list reference |
| [mqclass/zapret2-gui](https://github.com/mqclass/zapret2-gui) | main | MIT | Strategy catalog |
| [refraction-networking/utls](https://github.com/refraction-networking/utls) | master | BSD-3-Clause | TLS fingerprint reference |
| [rustls/rustls](https://github.com/rustls/rustls) | main | NOASSERTION | TLS library |
| [WireGuard/wintun](https://github.com/WireGuard/wintun) | master | NOASSERTION | Driver |
| [basil00/WinDivert](https://github.com/basil00/WinDivert) | master | NOASSERTION | Driver |

FlClashX `core/go.mod` at `a9bbb66f6aec3e47359819c93976148a1d270e23` requires Mihomo v1.19.34 (resolved tag `0455d0d28c4526937380505153e65a843d8612e8`) and mipstack commit `9a80a07eaeb9404d5bf416288e989ba7f947c899`; it also replaces sing-tun with its local third-party tree. These pins are the initial assembly reference, not a separate vanilla Mihomo tracking policy. FlClashZ (GPL-3.0 metadata) is a UI/behavior reference; do not modify it.

Before adapting any code, data or driver, review the license texts at the specific revisions, notices, per-file exceptions, dependencies, compatibility, source-distribution and driver redistribution obligations. NOASSERTION and absent metadata mean unresolved, not permission. Project LICENSE and third-party notices remain pending this review. Do not publish distributable releases or copy upstream code/resources until resolved.
