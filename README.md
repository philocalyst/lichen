# Welcome to Lichen.

[![Language Version](https://badgen.net/static/Rust/2024/orange)](https://developer.apple.com/macOS)
[![Apple Platform](https://badgen.net/badge/icon/macOS%2010.15+?icon=apple&label)](https://developer.apple.com/macOS)
[![Windows Platform](https://badgen.net/badge/icon/Windows%2010+?icon=windows&label)](https://developer.apple.com/macOS)
[![Linux Platform](https://badgen.net/badge/icon/Linux?icon=gnome&label)](https://developer.apple.com/macOS)

Lichen(lic) is a command-line tool designed to help manage license files and headers within your software projects. It simplifies generating full license texts (like `LICENSE`) and applying standardized license header comments to your source code files, ensuring compliance and consistency with patterns, configs, and templates. Supporting 600 licenses and counting!.

<img src="Trailer.gif">

## Summary

Lichen provides the following core functionalities:

* **License Generation (`gen`):** Creates full license text files based on SPDX identifiers (e.g., `MIT`, `Apache-2.0`). It fetches standard templates and populates them with author, year, and contributor information.
* **Header Application (`apply`):** Scans source files and adds or replaces license headers using appropriate comment syntax for various programming languages. It intelligently handles existing headers and respects `.gitignore` patterns.
* **Configuration (`init`, `.lichen.toml`):** Allows project-specific settings via a `.lichen.toml` file, including default license choices, author information, target file patterns, and exclusion rules. CLI arguments override configuration settings. Setup a config once, and run simply "lichen gen" or "lichen apply"
* **Multiple License Support** Double (or more?) license your code, with patterns and regex allowing you to target specific areas with specific licenses (Keep your code compliant when pulling in licensed snippets)

## Get Started

Ready to integrate Lichen into your project? Jump to the [Installation](#install) section to learn how to install the tool.

## Coming SOON
- Contributor option
- Full date option (Currently just trims to year)
- Config option (To pass in from wherever you are.)
- Uploading to all major package managers

## Tutorial

Lichen offers several commands to manage licenses.

### Initializing Configuration (Optional)

You can create a default configuration file (`.lichen.toml`) in your project root:

```shell
lichen init
# Or specify a path: lichen init path/to/project
````

This creates a `.lichen.toml` file where you can define default behaviors and license settings.

### Generating a License File

To generate a standard `LICENSE` file (e.g., MIT license) in the current directory:

```shell
# Using CLI arguments only
lic gen MIT --authors "Your Name:your.email@example.com" --date 2025

# Using configuration from .lichen.toml (if `id = "MIT"` is set)
lic gen
```

  * Replace `MIT` with the desired [SPDX license identifier](https://spdx.org/licenses/).
  * The `--authors` flag accepts comma-separated entries in the format `NAME[:EMAIL]`. And you can include more than one author with a comma separator.
  * The `--date` flag accepts `YYYY` or `YYYY-MM-DD`. If omitted, the current year is used.
  * Specify target directories or files after the license ID (defaults to `.`).
  * Use `--multiple` if you need to generate license files for multiple licenses. (Separate commands for each license if using the CLI)

### Applying License Headers

To add license headers to source files (e.g., apply Apache-2.0 header):

```shell
# Apply header based on CLI arguments, modifying files in place
lic apply Apache-2.0 --contributors "CONTRIBUTOR:contrib@example.com" --date 2025 src/ tests/

# Apply header(s) based on .lichen.toml configuration, modifying files in place
lic apply
```

> [!TIP]
> The apply supports two patterns of declaring specificity, designed to work together. The first is the target method, which takes filepaths to operate on, the default is the current directory. The second is exclude, which takes patterns to avoid operating on. Used together, you can create robust configuration easily.

  * Specify the license ID and optionally `--authors` and `--date`.
  * Specify target directories or files (defaults to `.`).
  * Use `--exclude <REGEX>` to provide a custom regex for excluding files/directories.
  * Use `--all` to ignore `.gitignore` and default ignore patterns.
  * Use `--prefer-block` to use block comments (`/* ... */`) instead of line comments (`// ...`) when available for the language.

### Configuration (`.lichen.toml`)

Lichen can be configured using a `.lichen.toml` file in your project root.

**Example `.lichen.toml`:**

```toml
# Global settings
prefer_block = false       # Default: Prefer line comments for headers
multiple = false           # Default: Generate 'LICENSE', not 'MIT_LICENSE' etc. Don't replicate headers.
all = false  # Default: Respect .gitignore and other exclude patterns (Below)

# Global exclude patterns (applied to all [[license]] blocks)
# Uses Rust Regex syntax.

exclude = [
  "\\.git/.*",             # Ignore Git directory
  ".*\\.lock",             # Ignore lock files (Cargo.lock, package-lock.json)
  "target/.*",             # Ignore Rust target directory
  "dist/.*",               # Ignore distribution directory
  "README.*",              # Ignore README files
  "LICENSE.*",             # Ignore top-level LICENSE files
  "\\.github/.*",          # Ignore GitHub workflow files
  "docs/.*\\.md",          # Ignore Markdown files in docs/
]

# Configuration for the MIT license
[[license]]
id = "MIT"                 # SPDX identifier
targets = ["src/", "examples/"] # Apply only to these directories/files
date = "2025-01-01"        # Specific date for this license block
exclude = ["src/third_party/.*"] # Specific exclusions for this license

[[license.authors]]
name = "Core Dev"
email = "core@example.com"

[[license.authors]]
name = "Another Dev"


# Configuration for the Apache-2.0 license (e.g., for specific modules)
[[license]]
id = "Apache-2.0"
targets = ["crates/apache_mod/"]
# Authors and date can be inherited globally or omitted if not needed specifically here

# [[license.authors]] # Optional: Override authors for this block
# name = "Apache Module Dev"
```

**Configuration Options:**

  * **Global:**
      * `prefer_block` (bool, optional): Prefer block comments for headers. Defaults to `false`. CLI `--prefer-block` overrides.
      * `multiple` (bool, optional): Generate `ID_LICENSE` instead of `LICENSE` (for `gen`), process all `[[license]]` blocks (for `apply`). Defaults to `false`. CLI `--multiple` overrides.
      * `ignore_git_ignore` (bool, optional): Ignore `.gitignore` content. Defaults to `false`. CLI `--all` overrides.
      * `exclude` (array of strings, optional): Global regex patterns for excluding files/directories. Applied before per-license excludes.
  * **Per-License (`[[license]]`):**
      * `id` (string, required): The SPDX license identifier (e.g., "MIT", "Apache-2.0").
      * `targets` (array of strings, optional): Specific files or directories this license block applies to. Defaults to `["."]` (current directory) if omitted entirely across CLI and all config blocks.
      * `authors` (array of tables, optional): List of authors (`{ name = "...", email = "..." }`). Overrides global authors if specified. CLI `--authors` overrides.
      * `date` (string `YYYY` or `YYYY-MM-DD`, optional): Copyright date. CLI `--date` overrides. Defaults to the current year/date.
      * `exclude` (string, optional): Regex pattern for additional exclusions specific to this license block. Applied *after* global excludes.

## Design Philosophy

Lichen aims to be:

  * **Correct:** Uses standard SPDX identifiers and fetches corresponding templates. Applies language-appropriate comment syntax.
  * **Configurable:** Offers flexibility through `.lichen.toml` for project-wide defaults and specific overrides, while also allowing CLI control.
  * **Integrated:** Respects `.gitignore` by default for intuitive exclusion handling.
  * **Robust:** Built with Rust for performance and reliable file processing. Handles different comment styles (line vs. block) and file encodings (UTF-8 assumed).

## Building and Debugging

This project uses `just` as a command runner (see `justfile`).

**Prerequisites:**

  * Rust toolchain (latest stable recommended): [rustup.rs](https://rustup.rs/)
  * `just` (optional but recommended): `cargo install just`

**Common Commands:**

```shell
# Check the code for errors without compiling fully
just check        # Debug check
just check-release # Release check (slower, more thorough)

# Build the project
just build        # Debug build
just build-release # Release build (optimized)

# Run the executable
just run -- --help      # Run debug build with args (--help)
just run-release -- gen MIT # Run release build with subcommand and args

# Run tests
just test

# Format code
just fmt

# Lint code
just lint        # Debug lint
just lint-release # Release lint
just lint-fix     # Attempt to automatically fix lint issues

# Generate documentation
just doc       # Build docs
just doc-open  # Build and open docs in browser

# Clean build artifacts
just clean     # Remove target/ directory
just clean-all # Remove target/, Python venv, etc.
```

## Install

### From Source

Ensure you have the Rust toolchain installed.

```shell
# Build and install using cargo (Installing just the public binary, not any helper)
cargo install --git https://github.com/philocalyst/lichen/ --bin lic
```

The `lichen` binary will be installed in your Cargo bin directory (usually `~/.cargo/bin/`). Ensure this directory is in your system's `PATH`.

### Pre-built Binaries

Pre-compiled binaries for major platforms (Linux, macOS, Windows) are available on the [GitHub Releases page](https://github.com/philocalyst/lichen/releases) under assets. Download the appropriate archive for your system, extract it, and place the `lichen` executable in a directory included in your system's `PATH`. You will also find `.sha256` files to verify the download integrity.

## Contributing
This is my first large rust project, and I'm sure there's so, so, so, much I could be doing better when it comes to following idioms and best practices, and I'm sure there's a ton of small bugs that are in my code.. right now 🥺

Because of that, the barrier to contributing is very low (For now haha), and if you notice something small or large (Could be code issue, design oddity, documentation lack etc.), just file an issue and I'll do the job of making sense of it. Pull requests can feel daunting, so if you *want* to contribute some code, you could even just drop an email to me (Found in my profile) with a line or two with some context :)

Some low hanging fruit are the parse_spdx library and the language generator (both of which no longer work/aren't in use), which I shamelessly generated to solve the problem once, and now represent the biggest pieces of debt in my project. I'll get around to integrating these as soon as possible, but are my priority after the promised features. If you want to jump that gun, please, by all means.

## Changelog

Notable changes to the project are documented in the [CHANGELOG.md](CHANGELOG.md) file.

## Libraries Used

Lichen builds upon several great Rust crates, including:

  * **CLI:** `clap` (argument parsing), `clap-verbosity-flag`
  * **File System & Paths:** `walkdir`, `ignore`, `directories`, `tempfile`
  * **Configuration & Serialization:** `serde`, `toml`, `serde_yaml`, `serde_json`, `serde_regex`
  * **Templating:** `handlebars`
  * **Text & Regex:** `regex`, `heck`
  * **Date/Time:** `jiff`, `chrono`
  * **Async:** `tokio`, `futures`
  * **Logging:** `log`, `env_logger`
  * **Internal:** `spdx_parser` (for processing SPDX template files), `metadata-gen` (build-time metadata extraction)

The comment token generation script (`scripts/parse_comments`) uses Python with `click` and `toml` (or `tomllib`).

## Acknowledgements

  * License template data is sourced/adapted from the [SPDX License List](https://spdx.org/licenses/).
  * Language comment token information is derived from the [Helix editor's](https://helix-editor.com/) language configuration.
  * Part of the inspiration for the project came from the [Licensure](https://github.com/chasinglogic/licensure) project.

## License

Lichen is distributed under the terms of the MIT License. See the [LICENSE](https://spdx.org/licenses/MIT.html) file for details.

