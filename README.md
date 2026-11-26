<p align="center">
  <img src="assets/logo.png" width="160" alt="Hyaline logo: a glass letter H">
</p>

<h1 align="center">Hyaline</h1>

<p align="center">
  <em>hyaline</em> (adj.) — glassy, transparent.<br>
  A tiny, fast Windows 11 taskbar customizer.
</p>

Make the taskbar transparent, blurred, acrylic or tinted, and switch looks automatically
based on what's on screen.

## Goals

- **Very light** — near-zero CPU at idle, tiny memory footprint, single small exe.
- **Just works** — Windows 11, multi-monitor, survives Explorer restarts.
- **Simple** — tray icon to configure, sensible defaults, no bloat.

## Status

Early development: Phases 0–2 done (foundation, research, Explorer helper). Roadmap: [`docs/PLAN.md`](docs/PLAN.md).

## Build from source

Requires [rustup](https://rustup.rs/) and Visual Studio Build Tools with the
"Desktop development with C++" workload (MSVC + Windows SDK). The Rust version
is pinned in `rust-toolchain.toml` and installed automatically.

```sh
cargo build --release
```

## License

[MIT](LICENSE)
