#!/usr/bin/env just

# ▰▰▰ Settings ▰▰▰ #
set shell := ["bash", "-euo", "pipefail", "-c"]
set windows-shell := ["C:/Program Files/Git/usr/bin/bash.exe", "-euo", "pipefail", "-c"]
set dotenv-load := true
set allow-duplicate-recipes := true

# ▰▰▰ Variables ▰▰▰ #
project_root    := justfile_directory()
output_directory := project_root + "/dist"

system := `rustc --version --verbose |  grep '^host:' | awk '{print $2}'`
target_dir      := project_root + "/target"
main_package      := "lic"
main_bin_flag     := "--bin" + main_package
spdx_parser_pkg := "spdx_parser"

py_script_dir := project_root + "/scripts/parse_comments"
py_script     := py_script_dir + "/main.py"
json_output_rel := "../../lic/src/comment-tokens.json"
json_output     := py_script_dir + "/" + json_output_rel

release_flag   := "--release"
workspace_flag := "--workspace"
all_flag       := "--all"
verbose_flag   := "-vv"

[doc('List all available recipes (default action)')]
default:
    just --list {{justfile()}}

# ▰▰▰ Build & Check ▰▰▰ #

[doc('Check workspace for compilation errors')]
[group('build')]
check:
    @echo "🔎 Checking workspace..."
    cargo check {{workspace_flag}}

[doc('Check workspace for compilation errors (release mode)')]
[group('build')]
check-release: 
    @echo "🔎 Checking workspace (release)..."
    cargo check {{workspace_flag}} {{release_flag}}

[doc('Build workspace in debug mode for specified target')]
[group('build')]
build target="aarch64-apple-darwin" package=(main_package):
    @echo "🔨 Building workspace (debug)..."
    cargo build {{workspace_flag}} --bin {{package}} --target {{target}}

[doc('Build workspace in release mode for specified target')]
[group('build')]
build-release target=(system) package=(main_package):
    @echo "🚀 Building workspace (release) for {{target}}…"
    cargo build {{workspace_flag}} {{release_flag}} --bin {{package}} --target {{target}}

# ▰▰▰ Packaging ▰▰▰ #

[doc('Package release binary with completions for distribution')]
[group('packaging')]
package target=(system):
    #!/usr/bin/env bash
    just build-release {{target}}
    echo "📦 Packaging release binary…"

    ext=""; 
    if [[ "{{target}}" == *windows-msvc ]]; then 
        ext=".exe"; 
    fi; 

    full_name="{{output_directory}}/{{main_package}}-{{target}}"
    mkdir -p $full_name

    bin="target/{{target}}/release/{{main_package}}${ext}"; 
    out="${full_name}/{{main_package}}${ext}"; 

    # now copy all completion scripts
    comp_dir="target/{{target}}/release"
    completions=( lic.bash lic.elv lic.fish _lic.ps1 _lic )

    for comp in "${completions[@]}"; do
      src="$comp_dir/$comp"
      dst="${full_name}"/$comp
      if [[ -f "$src" ]]; then
        echo " - cp $src → $dst"
        cp "$src" "$dst"
      else
        echo "Warning: completion script missing: $src" >&2
      fi
    done

    if [[ ! -d "{{output_directory}}" ]]; then
        echo "Error: Output directory '{{output_directory}}' was not created properly" >&2
        exit 1
    fi
    
    echo " - cp $bin → $out"; 
    cp "$bin" "$out"; 

[doc('Generate SHA256, MD5, and BLAKE3 checksums for distribution files')]
[group('packaging')]
checksum directory=(output_directory):
    #!/usr/bin/env bash
    set -euo pipefail

    dir="{{directory}}"
    echo "🔒 Generating checksums in '$dir'…"

    if [ ! -d "$dir" ]; then
        echo "Error: '$dir' is not a directory." >&2
        exit 1
    fi

    cd "$dir" || {
        echo "Error: cannot cd to '$dir'" >&2
        exit 1
    }

    # Go ahead and remove any stales
    [ -f *.sum ] && rm *.sum

    # Creating just a single checksum file for all the files in this directory
    find . -maxdepth 1 -type f \
        ! -name "*.sum" \
        -exec sha256sum {} + \
      > SHA256.sum || {
        echo "Error: failed to write checksums.sha256" >&2
        exit 1
    }

    find . -maxdepth 1 -type f \
        ! -name "*.sum" \
        -exec md5sum {} + \
      > MD5.sum || {
        echo "Error: failed to write checksums.sha256" >&2
        exit 1
    }

    find . -maxdepth 1 -type f \
        ! -name "*.sum" \
        -exec b3sum {} + \
      > BLAKE3.sum || {
        echo "Error: failed to write checksums.sha256" >&2
        exit 1
    }

    echo "✅ checksums.sha256 created in '$dir'"

[doc('Compress all release packages into tar.gz archives')]
[group('packaging')]
compress directory=(output_directory):
    #!/usr/bin/env bash
    set -e
    
    echo "🗜️ Compressing release packages..."
    
    if [ ! -d "{{directory}}" ]; then
        echo "Error: Directory '{{directory}}' does not exist" >&2
        exit 1
    fi
    
    # Process each package directory
    find "{{directory}}" -mindepth 1 -maxdepth 1 -type d | while read -r pkg_dir; do
        pkg_name=$(basename "$pkg_dir")
        echo "Compressing package: $pkg_name"
        
        # Create archive of the entire directory
        tar -czf "$pkg_dir.tar.gz" -C "$(dirname "$pkg_dir")" "$pkg_name" || {
            echo "Error: Failed to create archive for $pkg_name" >&2
            exit 1
        }
        
        echo "✅ Successfully compressed $pkg_name"
    done
    
    echo "🎉 All packages compressed successfully!"

[doc('Complete release pipeline: build, checksum, and compress')]
[group('packaging')]
release: build-release
    just checksum

# ▰▰▰ Execution ▰▰▰ #

[doc('Run application in debug mode with optional arguments')]
[group('execution')]
run package=(main_package) +args="":
    @echo "▶️ Running {{package}} (debug)..."
    cargo run --bin {{package}} -- {{args}}

[doc('Run application in release mode with optional arguments')]
[group('execution')]
run-release package=(main_package) +args="":
    @echo "▶️ Running {{package}} (release)..."
    cargo run --bin {{package}} {{release_flag}} -- {{args}}

[doc('Run SPDX parser example in debug mode')]
[group('execution')]
run-example-spdx: 
    @echo "▶️ Running spdx_parser example (basic_conversion)..."
    cargo run --bin {{spdx_parser_pkg}} --example basic_conversion

[doc('Run SPDX parser example in release mode')]
[group('execution')]
run-example-spdx-release:
    @echo "▶️ Running spdx_parser example (basic_conversion, release)..."
    cargo run --bin {{spdx_parser_pkg}} {{release_flag}} --example basic_conversion

# ▰▰▰ Code Generation ▰▰▰

[doc('Generate comment-tokens JSON file from language configurations')]
[group('codegen')]
generate-comments:
    @echo "🔧 Generating comment‐tokens JSON..."
    @mkdir -p "{{py_script_dir}}"
    @uv run "{{py_script}}" > "{{json_output}}"

# ▰▰▰ External Resources ▰▰▰ #

[doc('Download SPDX license templates from official repository')]
[group('resources')]
download-templates:
    git init
    git remote add origin https://github.com/spdx/license-list-data.git
    git config core.sparseCheckout true
    echo "template/" >> .git/info/sparse-checkout
    git pull origin main

[doc('Download language configuration file from external source')]
[group('resources')]
download-languages:
    curl -f -L -O -X GET https://github.com/philocalyst/lang-config/releases/latest/download/languages.json
    mv languages.json /Users/philocalyst/Projects/lichen/lic/assets/comment-tokens.json

# ▰▰▰ Testing ▰▰▰ #

[doc('Run all workspace tests')]
[group('testing')]
test: 
    @echo "🧪 Running workspace tests..."
    cargo test {{workspace_flag}}

[doc('Run workspace tests with additional arguments')]
[group('testing')]
test-with +args: 
    @echo "🧪 Running workspace tests with args: {{args}}"
    cargo test {{workspace_flag}} -- {{args}}

# ▰▰▰ Code Quality ▰▰▰ #

[doc('Format all Rust code in the workspace')]
[group('quality')]
fmt:
    @echo "💅 Formatting Rust code..."
    cargo fmt {{all_flag}}

[doc('Check if Rust code is properly formatted')]
[group('quality')]
fmt-check:
    @echo "💅 Checking Rust code formatting..."
    cargo fmt {{all_flag}} -- --check

[doc('Lint code with Clippy in debug mode')]
[group('quality')]
lint:
    @echo "🧹 Linting with Clippy (debug)..."
    cargo clippy {{workspace_flag}} -- -D warnings

[doc('Lint code with Clippy in release mode')]
[group('quality')]
lint-release:
    @echo "🧹 Linting with Clippy (release)..."
    cargo clippy {{workspace_flag}} {{release_flag}} -- -D warnings

[doc('Automatically fix Clippy lints where possible')]
[group('quality')]
lint-fix:
    @echo "🩹 Fixing Clippy lints..."
    cargo clippy {{workspace_flag}} --fix --allow-dirty --allow-staged

# ▰▰▰ Documentation ▰▰▰ #

[doc('Generate project documentation')]
[group('docs')]
doc:
    @echo "📚 Generating documentation..."
    cargo doc {{workspace_flag}} --no-deps

[doc('Generate and open project documentation in browser')]
[group('docs')]
doc-open: doc
    @echo "📚 Opening documentation in browser..."
    cargo doc {{workspace_flag}} --no-deps --open

# ▰▰▰ Maintenance ▰▰▰ #

[doc('Extract release notes from changelog for specified tag')]
[group('maintenance')]
create-notes raw_tag outfile changelog:
    #!/usr/bin/env bash
    
    tag_v="{{raw_tag}}"
    tag="${tag_v#v}" # Remove prefix v

    # Changes header for release notes
    printf "# What's new\n" > "{{outfile}}"

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

      # Error handling
      END {
        if (found_section != 0) {
          # Print error to stderr
          print "Error: awk could not find section header ## [" tag "] in " changelog_file > "/dev/stderr"
          exit 1
        }
      }
    ' "{{changelog}}" >> "{{outfile}}"

    # Check if the output file has content
    if [[ -s {{outfile}} ]]; then
      echo "Successfully extracted release notes to '{{outfile}}'."
    else
      # Output a warning if no notes were found for the tag
      echo "Warning: '{{outfile}}' is empty. Is '## [$tag]' present in '{{changelog}}'?" >&2
    fi

[doc('Update Cargo dependencies')]
[group('maintenance')]
update:
    @echo "🔄 Updating dependencies..."
    cargo update

[doc('Clean build artifacts')]
[group('maintenance')]
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

[doc('Clean all artifacts including generated files and caches')]
[group('maintenance')]
clean-all: clean
    @echo "🧹 Removing generated JSON file..."
    rm -f "{{json_output}}"
    @echo "🧹 Cleaning Python virtual environment..."
    cd "{{py_script_dir}}" && rm -rf .venv
    @echo "🧹 Cleaning Python cache..."
    cd "{{py_script_dir}}" && rm -rf .uv_cache __pycache__

# ▰▰▰ Installation ▰▰▰ #

[doc('Build and install binary to system')]
[group('installation')]
install package=(main_package): build-release 
    @echo "💾 Installing {{main_package}} binary..."
    cargo install --bin {{package}}

[doc('Force install binary, overwriting existing installation')]
[group('installation')]
install-force package=(main_package): build-release
    @echo "💾 Force installing {{main_package}} binary..."
    cargo install --bin {{package}} --force

# ▰▰▰ Aliases ▰▰▰ #

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
