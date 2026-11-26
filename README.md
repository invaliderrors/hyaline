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

Early development: Phases 0–6 done (foundation, research, Explorer helper, clear taskbar, tray and settings, dynamic looks, hardening and performance). Roadmap: [`docs/PLAN.md`](docs/PLAN.md).

## Usage

Run `hyaline.exe`; an **H** icon appears in the notification area (it may be under the ^
overflow arrow). Click it, or select it with the keyboard, for the menu.

Each taskbar picks its look from what is happening on its monitor, highest priority first:
**Start menu**, **Search**, **Task View**, **Maximized window**, **Visible window**, **Desktop**.
Every situation has its own submenu:

- **On** — whether the situation is used (Desktop is always on). By default Desktop is clear and
  a maximized window gives Windows' own look; the others are off.
- **Appearance** — Normal (Windows' own look), Opaque, Clear, Blur, Acrylic. Blur and Acrylic need
  Windows' *Transparency effects* setting on; without it Windows draws them as a solid tint.
- **Color…** and **Opacity** — the tint used by Opaque, Clear, Blur and Acrylic.

Also in the menu: **Start with Windows**, and **Exit** (puts Windows' own taskbar back).

Settings are saved to `%APPDATA%\Hyaline\config.toml`.

## Build from source

Requires [rustup](https://rustup.rs/) and Visual Studio Build Tools with the
"Desktop development with C++" workload (MSVC + Windows SDK). The Rust version
is pinned in `rust-toolchain.toml` and installed automatically.

```sh
cargo build --release
```

## License

[MIT](LICENSE)
