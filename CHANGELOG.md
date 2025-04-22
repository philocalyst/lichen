# Changelog

All notable changes to this project are documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.2.9]:    https://github.com/philocalyst/lichen/compare/v0.2.8...v0.2.9  
[0.2.8]:    https://github.com/philocalyst/lichen/releases/tag/v0.2.8  
