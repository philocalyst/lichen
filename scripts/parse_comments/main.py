#!/usr/bin/env uv run
import json
import sys
from pathlib import Path

import click
from click_spinner import spinner

# --- TOML Parser Selection ---
try:
    import tomllib  # Python 3.11+
    _parser = tomllib
    _decode_error = tomllib.TOMLDecodeError
    _parser_name = "tomllib (stdlib)"
except ImportError:
    try:
        import toml

        _parser = toml
        _decode_error = toml.TomlDecodeError
        _parser_name = "toml (package)"
    except ImportError:
        _parser = None
        _decode_error = None
        _parser_name = None
# --- End Parser Selection ---


@click.command(context_settings={"help_option_names": ["-h", "--help"]})
@click.option(
    "-i",
    "--input",
    "input_file",
    type=click.File("r", encoding="utf-8"),
    default="-",
    show_default=True,
    help="TOML source: path or '-' to read from stdin.",
)
@click.option(
    "-o",
    "--output",
    "output_path",
    type=click.Path(dir_okay=False, writable=True, path_type=Path),
    default=Path("language_data.json"),
    show_default=True,
    help="Path to write the JSON output.",
)
def cli(input_file, output_path: Path):
    """
    Extract comment tokens and simple string file types from a Helix
    languages.toml (or stdin) and write them as JSON.
    """
    if _parser is None:
        click.secho(
            "Error: No TOML parser found. Install Python 3.11+ "
            "or `pip install toml`.",
            fg="red",
            err=True,
        )
        sys.exit(1)

    click.secho(f"Using TOML parser: {_parser_name}", fg="magenta")

    source_name = getattr(input_file, "name", "<stdin>")
    click.secho(f"Reading configuration from: {source_name}", fg="cyan")
    try:
        with spinner():
            toml_content = input_file.read()
    except Exception as e:
        click.secho(f"Error reading {source_name}: {e}", fg="red", err=True)
        sys.exit(1)

    click.secho("Parsing TOML data...", fg="cyan")
    try:
        with spinner():
            config = _parser.loads(toml_content)
    except _decode_error as e:
        click.secho(f"TOML parse error: {e}", fg="red", err=True)
        sys.exit(1)
    except Exception as e:
        click.secho(f"Unexpected parse error: {e}", fg="red", err=True)
        sys.exit(1)

    language_data = {}
    langs = config.get("language")
    if not (isinstance(langs, list) and langs):
        click.secho(
            "Warning: No '[[language]]' sections found in the config.",
            fg="yellow",
            err=True,
        )
    else:
        click.secho(f"Found {len(langs)} language entries.", fg="green")
        for entry in langs:
            if not isinstance(entry, dict):
                click.secho(
                    "Skipping non-table entry in 'language' list.",
                    fg="yellow",
                    err=True,
                )
                continue
            name = entry.get("name")
            if not name:
                click.secho(
                    "Skipping a language entry without a name.",
                    fg="yellow",
                    err=True,
                )
                continue

            data = {}
            # Single-line comment
            raw = entry.get("comment-token") or entry.get("comment-tokens")
            if isinstance(raw, str):
                data["comment_token"] = raw
            elif isinstance(raw, list) and raw:
                data["comment_token"] = raw[0]

            # Block comments
            block = entry.get("block-comment-tokens")
            if block is not None:
                data["block_comment_tokens"] = block

            # File types (strings only)
            fts = entry.get("file-types")
            if isinstance(fts, list):
                only_str = [x for x in fts if isinstance(x, str)]
                skipped = len(fts) - len(only_str)
                if skipped:
                    click.secho(
                        f"Skipped {skipped} non-string file-types for '{name}'",
                        fg="yellow",
                        err=True,
                    )
                data["file_types"] = only_str
            elif fts is not None:
                click.secho(
                    f"Ignoring non-list file-types for '{name}'",
                    fg="yellow",
                    err=True,
                )

            if data:
                language_data[name] = data
            else:
                click.secho(
                    f"No data for '{name}', skipping.", fg="yellow", err=True
                )

    click.secho(f"Writing JSON output to: {output_path}", fg="cyan")
    try:
        with spinner():
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text(
                json.dumps(language_data, ensure_ascii=False, indent=4),
                encoding="utf-8",
            )
    except Exception as e:
        click.secho(f"Error writing JSON: {e}", fg="red", err=True)
        sys.exit(1)

    click.secho("Done! JSON file generated successfully.", fg="green")


if __name__ == "__main__":
    cli()
