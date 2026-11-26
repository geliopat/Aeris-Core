# Aeris Core

**Breathe freely.** Independent Rust networking core for Windows x64. Development version 0.1.0. Initialization only: the CLI currently exposes help/version; VPN, DNS, DPI and service operations are not implemented. No network changes are made by the scaffold.

[Aeris client](https://github.com/geliopat/Aeris) · [Resources](https://github.com/geliopat/Aeris-Resources) · [Specification](docs/SPECIFICATION.md) · [Roadmap](docs/ROADMAP.md)

## Build the scaffold

```powershell
cargo build --workspace --locked --release
cargo test --workspace --locked
.\target\release\aeris-core.exe version
```

CLI installation layout: place aeris-core.exe in the core root, cli/aeris.cmd in its cli subdirectory; add only the cli directory to PATH, not the GUI directory. Installer registration is not implemented. The project license is pending upstream audit; no third-party code is included here.
