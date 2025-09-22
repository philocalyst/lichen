#!/usr/bin/env just

# --- Settings --- #
set shell := ["bash", "-euo", "pipefail", "-c"]
set windows-shell := ["C:/Program Files/Git/usr/bin/bash.exe", "-euo", "pipefail", "-c"]
set dotenv-load := true

# --- Variables --- #
project_root    := justfile_directory()
output_directory := project_root + "/dist"
build_directory := `cargo metadata --format-version 1 | jq -r .target_directory`

system := `rustc --version --verbose |  grep '^host:' | awk '{print $2}'`
main_package      := "lic"
    
[doc('List all available recipes')]
default:
    @just --list

# --- Build & Check --- #
[doc('Check workspace for compilation errors')]
[group('build')]
check:
    @echo "🔎 Checking workspace..."
    cargo check --workspace

[doc('Build workspace in debug mode')]
[group('build')]
build target="aarch64-apple-darwin" package=(main_package):
    @echo "🔨 Building workspace (debug)..."
    cargo build --workspace --bin {{package}} --target {{target}}

[doc('Build workspace in release mode')]
[group('build')]
build-release target=(system) package=(main_package):
    @echo "🚀 Building workspace (release) for {{target}}…"
    cargo build --workspace --release --bin {{package}} --target {{target}}

# --- Packaging --- #
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
    comp_dir="{{build_directory}}/{{target}}/release"
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

[doc('Generate checksums for distribution files')]
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

# --- Execution --- #
[doc('Run application in debug mode')]
[group('execution')]
run package=(main_package) +args="":
    @echo "▶️ Running {{package}} (debug)..."
    cargo run --bin {{package}} -- {{args}}

[doc('Run application in release mode')]
[group('execution')]
run-release package=(main_package) +args="":
    @echo "▶️ Running {{package}} (release)..."
    cargo run --bin {{package}} --release -- {{args}}

# --- Testing --- #
[doc('Run all workspace tests')]
[group('testing')]
test: 
    @echo "🧪 Running workspace tests..."
    cargo test --workspace

[doc('Run workspace tests with additional arguments')]
[group('testing')]
test-with +args: 
    @echo "🧪 Running workspace tests with args: {{args}}"
    cargo test --workspace -- {{args}}

# --- Code Quality --- #
[doc('Format all Rust code in the workspace')]
[group('quality')]
fmt:
    @echo "💅 Formatting Rust code..."
    cargo fmt 
    

[doc('Check if Rust code is properly formatted')]
[group('quality')]
fmt-check:
    @echo "💅 Checking Rust code formatting..."
    cargo fmt 
    

[doc('Lint code with Clippy in debug mode')]
[group('quality')]
lint:
    @echo "🧹 Linting with Clippy (debug)..."
    cargo clippy --workspace -- -D warnings

[doc('Automatically fix Clippy lints where possible')]
[group('quality')]
lint-fix:
    @echo "🩹 Fixing Clippy lints..."
    cargo clippy --workspace --fix --allow-dirty --allow-staged

# --- Documentation --- #
[doc('Generate project documentation')]
[group('common')]
doc:
    @echo "📚 Generating documentation..."
    cargo doc --workspace --no-deps

[doc('Generate and open project documentation in browser')]
[group('common')]
doc-open: doc
    @echo "📚 Opening documentation in browser..."
    cargo doc --workspace --no-deps --open

# --- Maintenance --- #
[doc('Extract release notes from changelog for specified tag')]
[group('common')]
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

# --- Installation --- #
[doc('Build and install binary to system')]
[group('installation')]
install package=(main_package): build-release 
    @echo "💾 Installing {{main_package}} binary..."
    cargo install --bin {{package}}

[doc('Force install binary')]
[group('installation')]
install-force package=(main_package): build-release
    @echo "💾 Force installing {{main_package}} binary..."
    cargo install --bin {{package}} --force

# --- External Resources --- #
[doc('Download SPDX license templates from official repository')]
[group('external')]
download-templates:
    mkdir templates
    cd templates
    git init
    git remote add origin https://github.com/spdx/license-list-data.git
    git config core.sparseCheckout true
    echo "template/" >> .git/info/sparse-checkout
    git pull origin main

[doc('Download language configuration file from external source')]
[group('external')]
download-languages:
    xh --download --output="./lic/assets/comment-tokens.json" https://github.com/philocalyst/lang-config/releases/latest/download/languages.json

# --- Aliases --- #
alias b    := build
alias br   := build-release
alias c    := check
alias t    := test
alias f    := fmt
alias l    := lint
alias lf   := lint-fix
alias cl   := clean
alias up   := update
alias i    := install
alias ifo  := install-force
alias rr   := run-release
