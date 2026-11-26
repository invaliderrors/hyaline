<p align="center">
  <img src="assets/logo.png" width="160" alt="Hyaline logo: a glass letter H">
</p>

<h1 align="center">Hyaline</h1>

<p align="center">A tiny, fast Windows 11 taskbar customizer.</p>

Make the taskbar clear, blurred, acrylic or tinted, and switch looks automatically based on
what's on screen. No admin rights, no network access, no telemetry.

## Install

1. Download the zip from the [latest release](https://github.com/invaliderrors/hyaline/releases/latest).
2. Extract it to a folder you keep, for example `%LOCALAPPDATA%\Programs\Hyaline`.
3. Run `hyaline.exe`.

Requires Windows 11 (x64). Hyaline is not code-signed yet, so Windows may show
*Windows protected your PC*: choose **More info**, then **Run anyway**.

## Usage

Click the **H** icon in the notification area to open the menu.

Each taskbar uses the look of the highest-priority situation on its monitor: Start menu, Search,
Task View, Maximized window, Visible window, Desktop. For each one you can set:

- **On/Off**
- **Appearance**: Normal, Opaque, Clear, Blur or Acrylic
- **Color** and **Opacity**

The menu also has **Start with Windows** and **Exit**, which brings back the normal taskbar.

## Known limitations

- With several monitors, the taskbar background is hidden on all of them at once.
- Blur and Acrylic need Windows' *Transparency effects* setting turned on.
- Exit Hyaline before updating it.

## Uninstall

Turn off **Start with Windows**, choose **Exit**, then delete the folder.

## Build from source

Requires [rustup](https://rustup.rs/) and Visual Studio Build Tools with the
"Desktop development with C++" workload.

```sh
cargo build --release
```

## License

[MIT](LICENSE)
