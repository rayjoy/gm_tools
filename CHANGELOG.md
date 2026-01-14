# Changelog

All notable changes to this project are recorded in this file.

## 2026-01-14
- Initial workspace sync & setup
  - Added project files: `Cargo.toml`, `Cargo.lock`, and source files under `src/`.
  - Added example binaries under `src/bin/`: `test_crash.rs`, `test_decrypt.rs`.

- Development environment fixes
  - Installed `rust-src` component for the active toolchain (`stable-x86_64-pc-windows-msvc`).
  - Ensured `C:\Users\zhang\.cargo\bin` is present in the user PATH (permanent).
  - Resolved `rust-analyzer` errors by adding workspace VSCode settings:
    - `.vscode/settings.json` with `rust-analyzer.rustupPath` and `rust-analyzer.rustcSource=discover`.
    - Prepend `.cargo\bin` to the rust-analyzer server environment via `rust-analyzer.server.extraEnv.PATH`.

- Documentation
  - Added `README.md` containing design, usage, build instructions, known issues and development notes.
  - Added this `CHANGELOG.md`.

Notes:
- The SM2 implementation relies on the `libsm` crate; certain library versions may have compatibility issues with short inputs. The code includes temporary compatibility workarounds (padding, format conversion) which are documented in `README.md`.
