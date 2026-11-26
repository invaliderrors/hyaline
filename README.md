# Hyaline

> *hyaline* (adj.) — glassy, transparent.

A tiny, fast Windows taskbar customizer: make the taskbar transparent, blurred,
acrylic or tinted, and switch looks automatically based on what's on screen.

## Goals

- **Very light** — near-zero CPU at idle, tiny memory footprint, single small exe.
- **Just works** — Windows 11, multi-monitor, survives Explorer restarts.
- **Simple** — tray icon to configure, sensible defaults, no bloat.

## Status

Early development: Phase 0 (foundation) done. Roadmap: [`docs/PLAN.md`](docs/PLAN.md).

## Build from source

Requires [rustup](https://rustup.rs/) and Visual Studio Build Tools with the
"Desktop development with C++" workload (MSVC + Windows SDK). The Rust version
is pinned in `rust-toolchain.toml` and installed automatically.

```sh
cargo build --release
```

## License

[MIT](LICENSE)
