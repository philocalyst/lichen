# -*- mode: justfile -*-

# ===== Settings =====
set shell := ["bash", "-euo", "pipefail", "-c"]
set dotenv-load := true
set allow-duplicate-recipes := true

# ===== Variables =====
project_root    := justfile_directory()
host_target     := `rustc -Vv | sed -n 's/^host: //p'`
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

build-release target=(host_target):
    @echo "🚀 Building workspace (release) for {{target}}…"
    cargo build {{workspace_flag}} {{release_flag}} --target {{target}}

    @echo "📦 Packaging release binary…"
    @mkdir -p dist

    # In a single shell invocation, detect .exe suffix and copy+checksum
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
      echo -n "🔒 sha256: "; \
      sha256sum "$out" | awk "{print \$1}" > "$out.sha256"; \
      echo "(written dist/{{lichen_pkg}}-{{target}}${ext}.sha256)"; \
    '

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

test: generate-comments
    @echo "🧪 Running workspace tests..."
    cargo test {{workspace_flag}}

test-with +args: generate-comments
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
