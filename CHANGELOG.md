# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-11-26

First release. Known limitations are listed in the
[README](https://github.com/invaliderrors/hyaline#known-limitations).

### Added

- Taskbar looks for Windows 11: Normal (Windows' own), Opaque, Clear, Blur and Acrylic, with any
  color and opacity.
- Dynamic looks: Start menu, Search, Task View, Maximized window, Visible window and Desktop each
  have their own look and can be turned on or off; every taskbar follows what is happening on
  its own monitor.
- Tray icon and menu (mouse and keyboard), Start with Windows, and Exit.
- Settings in `%APPDATA%\Hyaline\config.toml`; an invalid file falls back to defaults with a
  notice.
- Windows' own taskbar comes back whenever Hyaline exits or crashes. The look survives Explorer
  restarts, sleep, theme, transparency, display and DPI changes, and new monitors.
- Steps aside in high contrast, and pauses if Explorer keeps restarting.
- Light: the look is applied 20–35 ms after start, about 2 MB of memory, no CPU while nothing on
  screen changes, about 640 KB of binaries.
- Release zip with a SHA-256 checksum and build provenance attestation.

[Unreleased]: https://github.com/invaliderrors/hyaline/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/invaliderrors/hyaline/releases/tag/v1.0.0
