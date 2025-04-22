# Changelog

All notable changes to this project are documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.2.9]:    https://github.com/philocalyst/lichen/compare/v0.2.8...v0.2.9  
[0.2.8]:    https://github.com/philocalyst/lichen/releases/tag/v0.2.8  
