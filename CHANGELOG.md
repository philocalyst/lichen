# Changelog

All notable changes to this project are documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.2] – 2025-04-30

### Added
- Added `description` and `license` fields to the Cargo.toml manifest.

### Changed
- Renamed the package from `lic` to `lichenn` in both Cargo.toml and Cargo.lock.

### Fixed
- Removed the redundant `.exe` suffix being appended to Windows release artifacts in the GitHub Actions workflow.

## [1.1.1] – 2025-04-29

### Changed
- Consolidate `Cli` struct: removed its duplicate definition in `cli.rs`, moved the single source-of-truth into `models.rs`, and updated the import in `main.rs`.

### Fixed
- Improve `justfile` path handling: properly quote `full_name` and verify that `output_directory` is created (erroring out if it isn’t).

## [1.1.0] – 2025-04-29

### Added
- Bundle and install CLI shell completions (Bash, Zsh, Fish, PowerShell, Elvish) alongside release binaries, with correct output-directory handling.

### Changed
- Overhauled the `compress-binaries` recipe in the Justfile:
  • Archives entire package directories instead of individual executables  
  • Improves logging and error handling  
  • Supports a configurable output directory  
- Changed compress-binaries to `compress`

### Fixed
- Clippy errors in `lic/src/cli.rs` and removed the obsolete `License::iter` implementation in `lic/src/license.rs`.  
- `build.rs` warning output now omits redundant `cargo:warning=` prefixes and correctly logs `OUT_DIR` and `PROFILE`.  
- Release workflow and Justfile checksum tasks now:
  • Produce unified `.sum` files (SHA256, MD5, BLAKE3)  
  • Remove stale checksum files before generation  
  • Fail on any error instead of silently continuing  
- Updated the release step to use the new checksum method and upload `.sum` artifacts.

## [1.0.0] - 2025-04-29

### Added
- Configuration option
- New license templates
- A test suite
- Compress binaries recipe
- Clap-complete and completions

### Changed
- All struct definitions now reside in the models.rs file

### Removed
- Weird trailer

## [0.3.6] – 2025-04-27

### Added
- Added platform-support badges (Rust version, macOS, Windows, Linux) to README.md.  
- Recipes for downloading comment tokens and license data
- Acknowledged the [Licensure](https://github.com/chasinglogic/licensure) project in README.md.

### Changed
- Refactored CLI to leverage Clap’s argument groups and flattened common structs (`LicenseArgs`, `FileProcessingArgs`).
- Switched many logs from `info!` to `debug!` and `trace!` for more granular output.
- Enhanced `render_license` to use the full `Date` type (not just year) when generating copyright.
- Applied broad code formatting and idiomatic Rust improvements across modules (`app`, `cli`, `commands`, `utils`).
- Updated CI workflow: the packaging step now runs `just package` instead of `just build-release`.
- Improved Justfile:
  - `build-release` and `package` recipes now accept `--bin` and dynamic target flags.
  - Refined defaults, alias definitions, and checksum generation for clarity.
- Pinned all dependencies in Cargo.toml to exact versions; regenerated Cargo.lock with updated checksums.
- Brought CHANGELOG.md up to date with the current release structure.

### Deprecated
- Deprecated and removed the Python-based `scripts/parse_comments` comment-token generator (moved to its own repository).

### Fixed
- Corrected `cargo install` instructions in README.md to install the `lic` binary only.
- Fixed Justfile recipes for `run`, `run-release`, `install`, `install-force`, and `checksum` to reference the correct package names and flags.

## [0.3.5] – 2025-04-23
### Fixed
- CI workflow

## [0.3.4] – 2025-04-23

### Added
- Trailer

## [0.3.3] – 2025-04-23
- Stress release

## [0.3.2] – 2025-04-23

## Fixed
- "ignoreGitignore" option in config is now all
- Gitignore loading will not error if not a git repoistory

## Added
- License init command
- Short "A" option for "all" in apply
- All dotfiles defaults case

## Changed
- Default config has constructive comments
- Gen command is _gen
- Positional target argument for init


## [0.3.1] – 2025-04-23

### Changed
- Moved to a bundled-with-the-binary solution for managing files and directories
- Logic for determining the default comment token

### Fixed
- Justfile error handling (In the awk block)
- Typos and clippy errors
- Generate script interface

### Added
- Justfile recipe for compressing binaries
- License (haha!)
- Coming soon section in README
- Contributing section in README

## [0.3.0] – 2025-04-22

### Added
- Add `unapply` subcommand to remove previously applied license headers. Handles shebangs and correctly identifies headers marked during application.
- Add initial `README.md` with project summary, usage examples, configuration details, and build instructions.

### Removed
- Remove `--in-place` argument and configuration option from the `apply` command. File modification is now the default and only behavior for `apply`.

### Fixed
- Correct configuration file name from `lichen.toml` to `.lichen.toml` in documentation, error messages, and loading logic.
- Example toml file (excludes) to (exclude)
- Fix typo in `Commands::Unapply` enum variant.

## [0.2.9] – 2025‑04‑22

### Added
- CLI now supports parsing full dates (`YYYY‑MM‑DD`) in addition to year‑only inputs.
- Fallback to January 1 when given a year‑only string (e.g. `"2025"` → `2025‑01‑01`).
- Errors are now propagated for invalid date strings longer than four characters.

### Changed
- Refined CLI documentation comments:
  - Clarified license file generation help (`--generate-license`).
  - Updated default config initialization description.
  - Described ignore‑pattern behavior: `.gitignore` and default patterns are skipped.

## [0.2.8] – 2025‑04‑22
- This is when I started keeping a changelog

[Unreleased]: https://github.com/your-org/your-repo/compare/v1.1.2...HEAD
[1.1.2]: https://github.com/your-org/your-repo/compare/v1.1.1...v1.1.2
[1.1.1]: https://github.com/your-org/your-repo/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/your-org/your-repo/compare/v1.0.0...v1.1.0  
[1.0.0]: https://github.com/philocalyst/lichen/compare/v0.3.6...v1.0.0  
[0.3.6]: https://github.com/philocalyst/lichen/compare/v0.3.5...v0.3.6  
[0.3.5]: https://github.com/philocalyst/lichen/compare/v0.3.4...v0.3.5
[0.3.4]:    https://github.com/philocalyst/lichen/compare/v0.3.3...v0.3.4  
[0.3.3]:    https://github.com/philocalyst/lichen/compare/v0.3.2...v0.3.3  
[0.3.2]:    https://github.com/philocalyst/lichen/compare/v0.3.1...v0.3.2  
[0.3.1]:    https://github.com/philocalyst/lichen/compare/v0.3.0...v0.3.1  
[0.3.0]:    https://github.com/philocalyst/lichen/compare/v0.2.9...v0.3.0  
[0.2.9]:    https://github.com/philocalyst/lichen/compare/v0.2.8...v0.2.9  
[0.2.8]:    https://github.com/philocalyst/lichen/releases/tag/v0.2.8  
