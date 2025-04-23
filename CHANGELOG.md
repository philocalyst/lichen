# Changelog

All notable changes to this project are documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.3.4]:    https://github.com/philocalyst/lichen/compare/v0.3.3...v0.3.4  
[0.3.3]:    https://github.com/philocalyst/lichen/compare/v0.3.2...v0.3.3  
[0.3.2]:    https://github.com/philocalyst/lichen/compare/v0.3.1...v0.3.2  
[0.3.1]:    https://github.com/philocalyst/lichen/compare/v0.3.0...v0.3.1  
[0.3.0]:    https://github.com/philocalyst/lichen/compare/v0.2.9...v0.3.0  
[0.2.9]:    https://github.com/philocalyst/lichen/compare/v0.2.8...v0.2.9  
[0.2.8]:    https://github.com/philocalyst/lichen/releases/tag/v0.2.8  
