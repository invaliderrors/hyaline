# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Hyaline steps aside in high contrast (Windows' own taskbar) and comes back when it is off.
- Crash-loop protection: if Explorer restarts three times within two minutes with the helper
  loaded, Hyaline pauses and says so; changing any setting resumes.
- Updates are safe while an older helper is still loaded in Explorer: Hyaline leaves it alone
  and asks for an Explorer restart.
- The look is re-applied after resume from sleep and on a newly created taskbar (monitor
  plugged in).
- Measured performance: the look is applied 20–35 ms after start, a state switch takes a few
  milliseconds, and the release binaries total about 640 KB (see `docs/testing.md`).
- Project skeleton: cargo workspace, CI, MIT license.
- Dynamic looks: Start menu, Search, Task View, Maximized window, Visible window and Desktop,
  each with its own look and on/off, resolved per monitor with zero idle CPU.
- Settings format version 2 (one look per situation); version 1 files are migrated: their
  look becomes the Desktop look, and by default a maximized window now gives Windows' own look
  (turn "Maximized window" off to keep one look everywhere).
- Per-monitor DPI awareness and modern common controls (application manifest).
- Tray icon and menu: Normal, Opaque, Clear, Blur and Acrylic looks with color and opacity,
  Start with Windows, and Exit (restores Windows' own taskbar).
- Settings saved to `%APPDATA%\Hyaline\config.toml`; an invalid file falls back to defaults
  with a notification.
- Version information in `hyaline.exe`.
- Clear taskbar: a transparent accent on every taskbar, kept across theme, transparency and
  display changes and Explorer restarts; Windows' exact default comes back when Hyaline stops.
- App logo, used as the `hyaline.exe` icon and in the README.
- Explorer helper: hides the taskbar background and always restores it, including when
  Hyaline exits or crashes and after Explorer restarts.
