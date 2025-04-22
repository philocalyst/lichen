# -*- mode: justfile -*-

# ===== Settings =====
set shell := ["bash", "-euo", "pipefail", "-c"]
set windows-shell := ["C:/Program Files/Git/usr/bin/bash.exe", "-euo", "pipefail", "-c"]
set dotenv-load := true
set allow-duplicate-recipes := true

# ===== Variables =====
project_root    := justfile_directory()
output_directory := project_root + "/dist"

# Get host line with grep, get the value with awk (2nd)
# host_target := shell(
#   "rustc --version --verbose |  grep '^host:' | awk '{print $2}'" 
# )
target_dir      := project_root + "/target"
lichen_pkg      := "lichen"
spdx_parser_pkg := "spdx_parser"

py_script_dir := project_root + "/scripts/parse_comments"
py_script     := py_script_dir + "/main.py"
json_output_rel := "../../lichen/src/comment-tokens.json"
json_output     := py_script_dir + "/" + json_output_rel

release_flag   := "--release"
workspace_flag := "--workspace"
all_flag       := "--all"
verbose_flag   := "-vv"

# ===== Default =====
default: check

# ===== Build & Check =====

check:
    @echo "🔎 Checking workspace..."
    cargo check {{workspace_flag}}

check-release: 
    @echo "🔎 Checking workspace (release)..."
    cargo check {{workspace_flag}} {{release_flag}}

build:
    @echo "🔨 Building workspace (debug)..."
    cargo build {{workspace_flag}}

create-notes raw_tag outfile changelog:
    #!/usr/bin/env bash
    tag_v="{{raw_tag}}"
    tag="${tag_v#v}" # Remove prefix v

    # Changes header for release notes
    printf "## Changes\n" > "{{outfile}}"

    if [[ ! -f "{{changelog}}" ]]; then
      echo "Error: {{changelog}} not found." >&2
      exit 1
    fi

    echo "Extracting notes for tag: {{raw_tag}} (searching for section [$tag])"
    # Use awk to extract the relevant section from the changelog
    awk -v tag="$tag" '
      # start printing when we see "## [<tag>]" (escape brackets for regex)
      $0 ~ ("^## \\[" tag "\\]") { printing = 1; next }
      # stop as soon as we hit the next "## [" section header
      printing && /^## \[/       { exit }
      # otherwise, if printing is enabled, print the current line
      printing                    { print }
    ' "{{changelog}}" >> "{{outfile}}"

    # Check if the output file has content
    if [[ -s {{outfile}} ]]; then
      echo "Successfully extracted release notes to '{{outfile}}'."
    else
      # Output a warning if no notes were found for the tag
      echo "Warning: '{{outfile}}' is empty. Is '## [$tag]' present in '{{changelog}}'?" >&2
    fi

build-release target="aarch64-apple-darwin":
    @echo "🚀 Building workspace (release) for {{target}}…"
    cargo build {{workspace_flag}} {{release_flag}} --target {{target}}

    @echo "📦 Packaging release binary…"
    @mkdir -p dist

    @bash -euo pipefail -c '\
      ext=""; \
      if [[ "{{target}}" == *windows-msvc ]]; then \
        ext=".exe"; \
      fi; \
      bin="target/{{target}}/release/{{lichen_pkg}}${ext}"; \
      out="dist/{{lichen_pkg}}-{{target}}${ext}"; \
      \
      echo " - cp $bin → $out"; \
      cp "$bin" "$out"; \
      \
    '

checksum directory=(output_directory):
    @echo "🔒 Creating checksums..."
    find "{{directory}}" -type f ! -name "checksums.sha256" ! -name "README*" ! -name "*.sha256" -exec sh -c 'sha256sum "$1" > "$1.sha256"' _ {} \;
    @echo "✅ Checksums created!"

# ===== Run =====

run +args:
    @echo "▶️ Running {{lichen_pkg}} (debug)..."
    cargo run -p {{lichen_pkg}} -- {{args}}

run-release +args:
    @echo "▶️ Running {{lichen_pkg}} (release)..."
    cargo run -p {{lichen_pkg}} {{release_flag}} -- {{args}}

run-example-spdx: 
    @echo "▶️ Running spdx_parser example (basic_conversion)..."
    cargo run -p {{spdx_parser_pkg}} --example basic_conversion

run-example-spdx-release:
    @echo "▶️ Running spdx_parser example (basic_conversion, release)..."
    cargo run -p {{spdx_parser_pkg}} {{release_flag}} --example basic_conversion

# ===== Code Generation =====

generate-comments:
    @echo "🔧 Generating comment‐tokens JSON..."
    @mkdir -p "{{py_script_dir}}"
    @uv run "{{py_script}}" > "{{json_output}}"

# ===== Test =====

test: 
    @echo "🧪 Running workspace tests..."
    cargo test {{workspace_flag}}

test-with +args: 
    @echo "🧪 Running workspace tests with args: {{args}}"
    cargo test {{workspace_flag}} -- {{args}}

# ===== Format & Lint =====

fmt:
    @echo "💅 Formatting Rust code..."
    cargo fmt {{all_flag}}

fmt-check:
    @echo "💅 Checking Rust code formatting..."
    cargo fmt {{all_flag}} -- --check

lint:
    @echo "🧹 Linting with Clippy (debug)..."
    cargo clippy {{workspace_flag}} -- -D warnings

lint-release:
    @echo "🧹 Linting with Clippy (release)..."
    cargo clippy {{workspace_flag}} {{release_flag}} -- -D warnings

lint-fix:
    @echo "🩹 Fixing Clippy lints..."
    cargo clippy {{workspace_flag}} --fix --allow-dirty --allow-staged

# ===== Documentation =====

doc:
    @echo "📚 Generating documentation..."
    cargo doc {{workspace_flag}} --no-deps

doc-open: doc
    @echo "📚 Opening documentation in browser..."
    cargo doc {{workspace_flag}} --no-deps --open

# ===== Cleaning =====

clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

clean-all: clean
    @echo "🧹 Removing generated JSON file..."
    rm -f "{{json_output}}"
    @echo "🧹 Cleaning Python virtual environment..."
    cd "{{py_script_dir}}" && rm -rf .venv
    @echo "🧹 Cleaning Python cache..."
    cd "{{py_script_dir}}" && rm -rf .uv_cache __pycache__

# ===== Installation & Update =====

update:
    @echo "🔄 Updating dependencies..."
    cargo update

release: build-release
    just checksum
    
install: build-release
    @echo "💾 Installing {{lichen_pkg}} binary..."
    cargo install --path "{{project_root}}/{{lichen_pkg}}"

install-force: build-release
    @echo "💾 Force installing {{lichen_pkg}} binary..."
    cargo install --path "{{project_root}}/{{lichen_pkg}}" --force

# ===== Aliases =====

alias b    := build
alias br   := build-release
alias c    := check
alias cr   := check-release
alias t    := test
alias f    := fmt
alias l    := lint
alias lr   := lint-release
alias lf   := lint-fix
alias cl   := clean
alias cla  := clean-all
alias up   := update
alias i    := install
alias ifo  := install-force
alias rr   := run-release
alias rre  := run-example-spdx-release
alias gc   := generate-comments
