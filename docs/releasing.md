# Releasing

Releases are built only by the `Release` workflow (`.github/workflows/release.yml`) from a
version tag. Hyaline follows [Semantic Versioning](https://semver.org/).

1. On a branch: set `version` in the root `Cargo.toml` and the `hyaline-core` dependency
   versions in `crates/hyaline/Cargo.toml` and `crates/hyaline-tap/Cargo.toml`, run
   `cargo build` to update `Cargo.lock`, and move the `[Unreleased]` entries in `CHANGELOG.md`
   into a new `## [X.Y.Z] - YYYY-MM-DD` section (with its compare link at the bottom).
2. Run the manual checklists in `docs/testing.md` that cover the changes. Open a pull request
   and merge it once CI passes.
3. Optional: run the `Release` workflow manually on `main` (Actions → Release → Run workflow)
   and test the zip it uploads as an artifact. A manual run publishes nothing.
4. Tag and push:

   ```sh
   git tag -a vX.Y.Z -m "Hyaline X.Y.Z"
   git push origin vX.Y.Z
   ```

   The workflow refuses a tag that differs from the workspace version, then checks, builds,
   attests and publishes the release with that version's CHANGELOG section as its notes.
5. Download the zip from the release, compare `Get-FileHash` with `SHA256SUMS.txt`, and run it.
6. winget: copy `packaging/winget/invaliderrors.Hyaline/<previous>` to a folder for the new
   version, update `PackageVersion`, `ReleaseDate`, `InstallerUrl`, `InstallerSha256` (from
   `SHA256SUMS.txt`) and `ReleaseNotesUrl`, run `winget validate --manifest <folder>`, commit it,
   and submit it to [`microsoft/winget-pkgs`](https://github.com/microsoft/winget-pkgs) under
   `manifests/i/invaliderrors/Hyaline/X.Y.Z/`.
