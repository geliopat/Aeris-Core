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

## License text follow-up (2026-11-26)

Read at the recorded revisions, not inferred from GitHub license metadata:

| Source | Text | Finding and packaging constraint |
|---|---|---|
| Zapret2 | [docs/LICENSE.txt](https://github.com/bol-van/zapret2/blob/93fbe9af8110107f44b70dba1ee956a0902f65b6/docs/LICENSE.txt) | MIT; preserve copyright and notice; check exceptions and bundled resources separately. |
| Flowseal | [LICENSE.txt](https://github.com/Flowseal/zapret-discord-youtube/blob/ffce9179b2b1f2aeb961f4d17274ada3b6d6bea5/LICENSE.txt) | MIT for covered work; explicitly distinguishes bol-van binaries and WinDivert licensing. Do not import bundled executables wholesale. |
| rustls | [LICENSE](https://github.com/rustls/rustls/blob/e11f98090bee85e6cb2ca03b11819fd145668795/LICENSE) | Apache-2.0 OR MIT OR ISC, subject to dependency/per-file review. |
| Wintun source | [COPYING](https://github.com/WireGuard/wintun/blob/ec0a6b98456fe1ba52567bb2add4bbf5f64315a1/COPYING) | GPL v2 text. Inspect per-file notices; distinguish source from official signed binaries. |
| Wintun official binaries | [prebuilt-binaries-license.txt](https://github.com/WireGuard/wintun/blob/ec0a6b98456fe1ba52567bb2add4bbf5f64315a1/prebuilt-binaries-license.txt) | Separate license permits distribution alongside software using the permitted API, with restrictions; review actual archive license, keep notices and use unmodified binaries via documented API. |
| WinDivert | [LICENSE](https://github.com/basil00/WinDivert/blob/4f311999b0d93b88f09450bb56ba9cc8cb028d9c/LICENSE) | LGPL v3 OR GPL v2; use separate dynamically loaded official signed binaries, review corresponding-source and notice obligations before distribution. |

Preliminary project-license candidate: GPL-3.0 for Aeris and Aeris-Core if compatible with all adapted source and chosen driver distribution terms. The client and core are separate works with a documented IPC boundary; no license assignment is made in this change. Resource repository requires per-artifact licenses (lists and geodata may differ). This is not a completed legal audit; review the exact source files and actual distributable binaries before publishing. No upstream code or drivers have been copied.

При достижении этапов 6 и 10 проверить лицензии конкретных ревизий официальных AmneziaWG и трёх эталонов DPI из [UPSTREAM.md](UPSTREAM.md). Для заимствований и распространяемых стратегий, Lua, списков и драйверных компонентов отдельно зафиксировать точное происхождение, разрешение на использование и распространение, обязательные уведомления и подтверждённую совместимость. Приведённые выше лицензии относятся к указанным ревизиям.
