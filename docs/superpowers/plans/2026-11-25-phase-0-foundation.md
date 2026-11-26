# Phase 0 — Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A fresh, latest-stable Rust toolchain and an empty three-crate workspace that builds, lints and tests green locally and in GitHub Actions, published as a public MIT repository.

**Architecture:** Cargo workspace with `crates/hyaline` (GUI-subsystem exe), `crates/hyaline-tap` (cdylib, later injected into Explorer) and `crates/hyaline-core` (pure shared lib). No product code and no dependencies yet — later phases add them when used.

**Tech Stack:** Rust (rustup, latest stable, edition 2024), latest Visual Studio Build Tools (MSVC + Windows 11 SDK), GitHub Actions, `gh` CLI.

**Spec:** `docs/superpowers/specs/2026-11-25-hyaline-design.md` (§7 Tooling, §8 Phase 0)

## Global Constraints

- Commit messages: Conventional Commits; **no `Co-Authored-By` or any AI attribution** (`CLAUDE.md`).
- Rust edition `2024`; toolchain = latest stable at execution time, pinned to the exact version in `rust-toolchain.toml`.
- Lints: `clippy::pedantic` on; warnings are errors in CI (`-D warnings`).
- Zero dependencies in Phase 0.
- License: MIT, copyright holder `A. Mestra` (from `git config user.name`), year 2026.
- GitHub repo: `invaliderrors/hyaline`, **public** (name verified free on 2026-11-25).
- Use the latest released major of every GitHub Action, pinned to its full commit SHA with the version in a trailing comment.

## Review Focus

1. `cargo` on PATH resolves to the old standalone Rust 1.93 instead of rustup → wrong compiler. Expect `where.exe cargo` to list `%USERPROFILE%\.cargo\bin\cargo.exe` first (Task 1).
2. Linker not found or picks a broken VS install → build fails. Expect a release build of a hello-world to link with the new Build Tools (Task 1).
3. Local and CI toolchains drift → "works on my machine". Expect CI to install exactly the version in `rust-toolchain.toml` (Task 3).
4. `Cargo.lock` not committed → `--locked` fails in CI. Expect it tracked in git (Task 2).
5. `hyaline-tap` not built as a DLL → Phase 3 surprise. Expect `target/release/hyaline_tap.dll` to exist after a release build (Task 2).

---

### Task 1: Latest toolchain on this machine

Machine setup only — no repo files. Uninstalling software changes the system: **ask the owner before Step 1.**

- [ ] **Step 1: Remove the standalone Rust that shadows rustup**

Uninstall "Rust stable MSVC 1.93" (Settings → Apps, or `winget uninstall` by its listed name). rustup already exists at `%USERPROFILE%\.cargo\bin`.

- [ ] **Step 2: Update rustup and stable**

Run: `rustup self update; rustup update stable; rustup component add rustfmt clippy`
Then in a new shell: `where.exe cargo` and `rustc -V`
Expected: first `cargo` hit is under `.cargo\bin`; `rustc` prints the latest stable (≥ 1.93). Record the exact version for Task 2.

- [ ] **Step 3: Install the latest Visual Studio Build Tools**

Run `winget search "Visual Studio Build Tools"` and pick the newest `Microsoft.VisualStudio.*BuildTools` id. Install with workloads `Microsoft.VisualStudio.Workload.VCTools` + `--includeRecommended` (brings the latest Windows 11 SDK), via `winget install <id> --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"`. Leave VS 2019 Build Tools installed unless the owner asks to remove it.

- [ ] **Step 4: Verify a link end-to-end**

Run in the scratchpad: `cargo new hello && cargo build --release --manifest-path hello/Cargo.toml && hello/target/release/hello.exe`
Expected: prints `Hello, world!`. Delete `hello` afterwards.

---

### Task 2: Workspace skeleton

**Files:**
- Create: `Cargo.toml` (workspace), `rust-toolchain.toml`, `Cargo.lock` (generated)
- Create: `crates/hyaline/Cargo.toml`, `crates/hyaline/src/main.rs`
- Create: `crates/hyaline-tap/Cargo.toml`, `crates/hyaline-tap/src/lib.rs`
- Create: `crates/hyaline-core/Cargo.toml`, `crates/hyaline-core/src/lib.rs`

**Interfaces:**
- Produces: package names `hyaline` (bin), `hyaline-tap` (lib name `hyaline_tap`, `crate-type = ["cdylib"]` → `hyaline_tap.dll`), `hyaline-core` (lib name `hyaline_core`). Later phases add code to these exact paths.

- [ ] **Step 1: Bootstrap commit on `main`**

Commit the existing docs first so the branch has a base:
```bash
git add .gitignore CLAUDE.md README.md docs
git commit -m "docs: add design, roadmap and project rules"
git switch -c chore/foundation
```

- [ ] **Step 2: Write the workspace files**

- `Cargo.toml`: `[workspace]` with `members = ["crates/*"]`, `resolver = "3"`; `[workspace.package]` with `edition = "2024"`, `license = "MIT"`, `version = "0.0.0"`, `rust-version` = the exact version from Task 1; `[workspace.lints.clippy]` `pedantic = { level = "warn", priority = -1 }`.
- `rust-toolchain.toml`: `channel = "<exact version from Task 1>"`, `components = ["rustfmt", "clippy"]`.
- Each crate `Cargo.toml`: inherits `edition`, `license`, `version`, `rust-version` with `.workspace = true`, and `[lints] workspace = true`.
- `crates/hyaline/src/main.rs`: `#![windows_subsystem = "windows"]` and an empty `fn main() {}`.
- `crates/hyaline-tap/src/lib.rs` and `crates/hyaline-core/src/lib.rs`: a single `//!` doc line stating the crate's purpose (from spec §5).

- [ ] **Step 3: Verify**

Run: `cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo build --workspace --release; cargo test --workspace`
Expected: all exit 0, no warnings; `target/release/hyaline.exe` and `target/release/hyaline_tap.dll` exist.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock rust-toolchain.toml crates
git commit -m "build: add cargo workspace with hyaline, hyaline-tap and hyaline-core"
```

---

### Task 3: License, changelog, README and CI

**Files:**
- Create: `LICENSE`, `CHANGELOG.md`, `.github/workflows/ci.yml`
- Modify: `README.md`

- [ ] **Step 1: Repo documents**

- `LICENSE`: standard MIT text, `Copyright (c) 2026 A. Mestra`.
- `CHANGELOG.md`: Keep a Changelog 1.1.0 format, `## [Unreleased]` → `### Added` → `- Project skeleton: cargo workspace, CI, MIT license.`
- `README.md`: add **Build from source** (`cargo build --release`, requires rustup + VS Build Tools with C++ workload) and **License** (MIT, link to `LICENSE`) sections.

- [ ] **Step 2: CI workflow**

`.github/workflows/ci.yml`, name `CI`, job id `check`, triggers `push` to `main` and `pull_request`; `permissions: contents: read`; `runs-on: windows-latest`. Steps:
1. `actions/checkout` (latest major, SHA-pinned).
2. `rustup toolchain install` (reads `rust-toolchain.toml`).
3. `cargo fmt --all --check`
4. `cargo clippy --workspace --all-targets --locked -- -D warnings`
5. `cargo build --workspace --release --locked`
6. `cargo test --workspace --locked`

- [ ] **Step 3: Verify locally**

Re-run the four cargo commands from Task 2 Step 3 with `--locked` added.
Expected: all exit 0.

- [ ] **Step 4: Commit**

```bash
git add LICENSE CHANGELOG.md README.md .github
git commit -m "ci: add CI workflow, MIT license and changelog"
```

---

### Task 4: Publish and prove CI

Creating a public repo publishes the code: **confirm with the owner before Step 1.**

- [ ] **Step 1: Create the repo and push `main`**

```bash
git switch main
gh repo create invaliderrors/hyaline --public --source . --remote origin --description "Tiny, fast Windows 11 taskbar transparency, blur and acrylic." --push
```

- [ ] **Step 2: Push the branch and open the PR**

```bash
git push -u origin chore/foundation
gh pr create --base main --title "chore: project foundation" --body "Phase 0 per docs/superpowers/plans/2026-11-25-phase-0-foundation.md"
```

- [ ] **Step 3: Verify CI**

Run: `gh pr checks --watch`
Expected: `check` passes. If it fails, fix on the branch and push until green.

- [ ] **Step 4: Merge and protect `main`**

`gh pr merge --squash --delete-branch`, then add a branch ruleset on `main` requiring the `check` status and pull requests (`gh api repos/invaliderrors/hyaline/rulesets` POST).
Expected: `gh api repos/invaliderrors/hyaline/rulesets` lists the ruleset; direct pushes to `main` are rejected.

- [ ] **Step 5: Update the roadmap**

Set Phase 0 to `done` in `docs/PLAN.md` via a small PR (`docs: mark phase 0 done`).
