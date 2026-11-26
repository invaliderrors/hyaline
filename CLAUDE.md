# Hyaline

Lightweight Windows taskbar customizer (transparency, blur, acrylic, tint). The quality bar is
production software shipped to the public: every change must be professional, deliberate and verified.

## Git commits

- NEVER add `Co-Authored-By` trailers (or any other attribution) for Claude or any other AI model to commit messages.
- NEVER add "Generated with Claude Code" or similar AI attribution to commit messages or pull request descriptions.
- Use [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `perf:`, `refactor:`, `docs:`, `test:`, `build:`, `ci:`, `chore:`.
  Subject in imperative mood, ≤ 72 chars; body explains *why*, not *what*.
- One logical change per commit. Never commit broken builds, failing tests, generated artifacts or secrets.
- `main` is always releasable. Work on short-lived branches (`feat/…`, `fix/…`) merged via pull request.

## Process

- Design before code: non-trivial work starts with a design doc in `docs/superpowers/specs/` and a plan.
- Test-driven development: write the failing test first, then the minimal code to pass it, then refactor.
- Never claim something works without running the verification (build, tests, lint) and reading the output.
- Keep `docs/PLAN.md`, `README.md` and `CHANGELOG.md` current with every user-visible change.

## Code quality

- Latest stable versions of the toolchain, SDKs and every dependency. Pin exact versions in lockfiles; update deliberately.
- Zero warnings: the formatter and linter run in their strictest practical mode, and warnings fail the build.
- Small, focused modules with one clear responsibility. Clear names over comments; comments explain *why*.
- No dead code, no commented-out code, no TODOs without a linked issue.
- Minimize dependencies. Every new dependency needs a reason (size, maintenance, license, security).
- All OS/API calls check and handle errors explicitly. No silent failures, no panics/crashes on recoverable errors.
- `unsafe` / raw Win32 calls are isolated in thin, documented wrapper modules with a stated safety contract.

## Windows-specific rules

- Never destabilize Explorer. Any code that touches `explorer.exe` must fail safe: on error, restore the default taskbar and back off.
- Survive Explorer restarts, sleep/resume, display changes, DPI changes, theme (light/dark) changes and monitor hot-plug.
- Per-monitor DPI aware (v2). Correct on every taskbar, including secondary monitors.
- Event-driven only: no polling loops or timers where a Windows event/notification exists. Idle CPU must be effectively zero.
- Restore the original taskbar appearance on exit, crash and uninstall.
- Respect accessibility settings (high contrast, "transparency effects" off, reduced motion).

## Performance

- Performance budgets (idle CPU, memory, binary size, startup time) are defined in the design doc and measured, not guessed.
- Any change that could affect a budget includes before/after numbers in its PR.

## Testing

- Unit tests for all logic that does not need a live desktop (config parsing, state machine, rule evaluation).
- Integration/manual test checklist in `docs/testing.md` for Windows-dependent behavior, run before every release.
- Bugs get a regression test before the fix.

## Security & privacy

- No telemetry, no network access, no data collection.
- Least privilege: runs as a normal user, never requires admin.
- Release binaries are built by CI from a tagged commit, with checksums published (code-signed when possible).
- Never commit secrets, certificates or private keys.

## CI & releases

- CI on every push/PR: format check, lint, build (release config), tests. Red CI blocks merging.
- [Semantic Versioning](https://semver.org/). `CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/).
- Releases are reproducible from a git tag.

## Documentation

- `README.md`: what it does, install, usage, build from source, license.
- Public modules and non-obvious functions have doc comments.
- Architecture decisions recorded in `docs/` with their reasoning.
