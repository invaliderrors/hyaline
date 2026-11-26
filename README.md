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

## Install

Requires Windows 11 (x64).

1. Download `hyaline-<version>-x64.zip` from the
   [latest release](https://github.com/invaliderrors/hyaline/releases/latest).
2. Optional: check the download. Its SHA-256 is in `SHA256SUMS.txt` next to it
   (`Get-FileHash .\hyaline-<version>-x64.zip`), and its build provenance can be verified with
   `gh attestation verify hyaline-<version>-x64.zip --repo invaliderrors/hyaline`.
3. Extract it to a folder you keep, for example `%LOCALAPPDATA%\Programs\Hyaline`, and run
   `hyaline.exe`.

The binaries are not code-signed, so the first run may show *Windows protected your PC*:
choose **More info**, then **Run anyway**.

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

## How it works

Windows 11 draws the taskbar background with XAML inside Explorer. Hyaline sets the taskbar
windows' accent (color, blur, acrylic) and loads a small helper, `hyaline_tap.dll`, into Explorer
through Windows' XAML Diagnostics interface to hide that background. The helper does nothing
else, and puts the background back as soon as Hyaline exits or crashes. Explorer keeps the helper
loaded until it restarts, so Hyaline loads it from a copy in `%LOCALAPPDATA%\Hyaline` and the
install folder stays free to update or delete.

No administrator rights, no network access, no telemetry.

## Known limitations

- Hiding the background applies to all taskbars at once. With several monitors, a taskbar that
  should show Windows' own look while another one is tinted shows Windows' accent without its
  background.
- While the mouse moves, Hyaline uses up to about 1 % of one CPU core: Windows reports every cursor
  move as a location change, which Hyaline needs to notice maximized windows. With nothing
  changing on screen it uses none.
- To update, choose **Exit** first (a running `hyaline.exe` cannot be replaced), then replace the
  files. If the new version's helper differs, Hyaline shows a notice until Explorer restarts or
  you sign out.

## Uninstall

1. In the tray menu, turn off **Start with Windows**, then choose **Exit**.
2. Delete the folder you extracted Hyaline to.
3. Optionally delete `%APPDATA%\Hyaline` (settings) and `%LOCALAPPDATA%\Hyaline` (the helper copy;
   if Explorer still has it loaded, delete it after signing out).

## Build from source

Requires [rustup](https://rustup.rs/) and Visual Studio Build Tools with the
"Desktop development with C++" workload (MSVC + Windows SDK). The Rust version
is pinned in `rust-toolchain.toml` and installed automatically.

```sh
cargo build --release
```

Releases are built by CI from version tags; see [`docs/releasing.md`](docs/releasing.md).
Roadmap and design: [`docs/PLAN.md`](docs/PLAN.md).

## License

[MIT](LICENSE)
