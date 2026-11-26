# Phase 7 — Release 1.0 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** v1.0.0 is built by CI from its git tag, published on GitHub with checksums and build provenance, and has a validated winget manifest ready to submit.

**Architecture:** A `release.yml` workflow builds, tests and packages on a `v*.*.*` tag (a manual run produces the same package as an artifact without publishing). The exe loads the helper from a content-named copy under `%LOCALAPPDATA%\Hyaline`, so Explorer never locks the install folder (winget upgrades and uninstalls replace that folder while Explorer keeps the old helper loaded until it restarts).

**Tech Stack:** Rust 1.100.0, GitHub Actions (`windows-latest`, `gh`), winget manifest schema 1.10.

**Spec:** `docs/superpowers/specs/2026-11-25-hyaline-design.md` (§8 Phase 7: release workflow from git tag, checksums, signing if a certificate is available, winget manifest, README).

## Global Constraints

- Commits: Conventional Commits, granular, **no AI attribution**. Branch `feat/phase-7-release`, PR, squash merge.
- Every third-party action pinned to a full commit SHA with its version in a comment; least-privilege `permissions`.
- No code-signing certificate: unsigned binaries, SHA-256 checksums plus GitHub build provenance attestation; README explains the SmartScreen prompt.
- Release binaries only from CI on a tag; the tag must equal the workspace version.
- Submitting to `microsoft/winget-pkgs` is an outward action under the owner's account: ask first.

## Owner-delegated rulings (made while planning)

- Package: `hyaline-<version>-x64.zip` with `hyaline.exe`, `hyaline_tap.dll`, `LICENSE`; `SHA256SUMS.txt` beside it. x64 only.
- winget: `PackageIdentifier: invaliderrors.Hyaline`, zip with a nested portable `hyaline.exe` (command alias `hyaline`).
- Helper copy named `hyaline_tap-<FNV-1a 64 of its bytes, 16 hex>.dll`; other copies are deleted on start when not in use. If copying fails, the DLL next to the exe is used.

## Review Focus

1. Started through a symlink (winget portable alias) → expect the helper found next to the real exe.
2. winget upgrade while Explorer has the old helper loaded → expect the install folder replaceable (nothing in it locked by Explorer), new exe working with the copy it makes.
3. Tag not matching the workspace version → expect the release job to fail before publishing.
4. Manual workflow run → expect a package artifact and no release, no tag.
5. Two copies of different builds in `%LOCALAPPDATA%\Hyaline` → expect the one in use kept, the other deleted.

---

### Task 1: Version 1.0.0

Workspace `version = "1.0.0"`, path dependency versions `1.0.0`, `Cargo.lock` updated. Commit `build: version 1.0.0`.

### Task 2: Helper copy outside the install folder (`hyaline-core::tap::tap_file_name`, TDD; exe)

`pub fn tap_file_name(contents: &[u8]) -> String` → `hyaline_tap-{fnv1a64:016x}.dll`; `pub fn is_tap_copy(name) -> bool`. Tests: FNV-1a vectors (`""` → `cbf29ce484222325`, `"a"` → `af63dc4c8601ec8c`) through the name; `is_tap_copy` accepts the produced names and rejects `hyaline_tap.dll`, other files. Exe `tap_file.rs`: find `hyaline_tap.dll` next to `current_exe()`, else next to its canonical path (symlink); copy to `%LOCALAPPDATA%\Hyaline\<name>` via a temp file + rename when missing; delete other copies (failures ignored: in use); return the copy, or the original on any error.

### Task 3: Release workflow

`.github/workflows/release.yml`: on `push: tags: ['v*.*.*']` and `workflow_dispatch`. Steps: checkout, toolchain, tag == version check (tags only), fmt, clippy, test, release build, package zip + `SHA256SUMS.txt`, `actions/attest-build-provenance` (tags only), upload artifact (manual runs), `gh release create` with notes extracted from the CHANGELOG section (tags only). Verify with a manual run on the branch; download and inspect the artifact.

### Task 4: winget manifest

`packaging/winget/invaliderrors.Hyaline/1.0.0/` (version, installer, defaultLocale). `InstallerSha256` filled after the release; `winget validate` passes.

### Task 5: Docs and release

README: Install (winget, zip download, SmartScreen, checksum check), Uninstall (Exit, then winget uninstall or delete folder; leftover `%APPDATA%\Hyaline`, `%LOCALAPPDATA%\Hyaline`, Start with Windows), Status → 1.0. `docs/releasing.md` (cut a release). CHANGELOG `[1.0.0] - date` user-facing section + links; PLAN Phase 7 done. Merge, tag `v1.0.0`, watch the workflow, download the release zip, verify checksum and run it. Fill the manifest SHA in a follow-up PR. Ask the owner before submitting to winget-pkgs.
