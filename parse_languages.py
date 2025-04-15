#!/usr/bin/env python3

import json
import argparse
import sys
from pathlib import Path

# --- TOML Parser Selection ---
# Use tomllib if available (Python 3.11+), otherwise try the 'toml' package
try:
    import tomllib
    _parser = tomllib
    _decode_error = tomllib.TOMLDecodeError
except ImportError:
    try:
        import toml
        _parser = toml
        _decode_error = toml.TomlDecodeError
    except ImportError:
        print("Error: Required TOML parser not found.", file=sys.stderr)
        print("Please install 'tomllib' (standard in Python 3.11+) or 'toml' (`pip install toml`).", file=sys.stderr)
        sys.exit(1)
# --- End TOML Parser Selection ---

def extract_comment_tokens(input_filepath: Path, output_filepath: Path):
    """
    Parses a TOML configuration file, extracts language comment tokens,
    and writes them to a JSON file.

    Args:
        input_filepath: Path to the input TOML configuration file.
        output_filepath: Path to the output JSON file.
    """
    print(f"Reading configuration from: {input_filepath}")
    try:
        toml_content = input_filepath.read_text(encoding='utf-8')
    except FileNotFoundError:
        print(f"Error: Input file not found: {input_filepath}", file=sys.stderr)
        sys.exit(1)
    except IOError as e:
        print(f"Error reading input file {input_filepath}: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred while reading {input_filepath}: {e}", file=sys.stderr)
        sys.exit(1)

    # Parse the TOML data
    try:
        config = _parser.loads(toml_content)
    except _decode_error as e:
        print(f"Error parsing TOML file {input_filepath}: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred during TOML parsing: {e}", file=sys.stderr)
        sys.exit(1)

    language_comments = {}

    # Check if the 'language' key exists and is a list
    if 'language' in config and isinstance(config['language'], list):
        print(f"Found {len(config['language'])} language definitions. Processing...")
        for lang_config in config['language']:
            # Ensure the item is a dictionary and has a 'name'
            if not isinstance(lang_config, dict):
                continue
            lang_name = lang_config.get('name')
            if not lang_name:
                continue # Skip languages without a name

            lang_data = {}

            # --- Extract Single-Line Comment Token ---
            # Helix uses both 'comment-token' (string) and 'comment-tokens' (list)
            comment_token_single = lang_config.get('comment-token')
            comment_tokens_list = lang_config.get('comment-tokens') # Note the 's'

            raw_comment_token = None
            if comment_token_single is not None:
                raw_comment_token = comment_token_single
            elif comment_tokens_list is not None:
                 raw_comment_token = comment_tokens_list
            else:
                 raw_comment_token = None # Explicitly none if neither key found

            if isinstance(raw_comment_token, str):
                 lang_data['comment_token'] = raw_comment_token
            elif isinstance(raw_comment_token, list) and raw_comment_token:
                 # Take the first token from the list as the primary single-line token
                 lang_data['comment_token'] = raw_comment_token[0]

            # --- Extract Block Comment Tokens ---
            block_tokens = lang_config.get('block-comment-tokens')
            if block_tokens is not None:
                # Can be a single dict {start="..", end=".."} or a list of such dicts
                lang_data['block_comment_tokens'] = block_tokens

            # Only add the language if comment info was found
            if lang_data:
                language_comments[lang_name] = lang_data
    else:
        print("Warning: No '[[language]]' sections found in the configuration.", file=sys.stderr)


    # Write the extracted data to the JSON file
    print(f"Writing extracted comment tokens to: {output_filepath}")
    try:
        with open(output_filepath, 'w', encoding='utf-8') as f:
            json.dump(language_comments, f, indent=4, ensure_ascii=False)
        print("Successfully wrote JSON output.")
    except IOError as e:
        print(f"Error writing output file {output_filepath}: {e}", file=sys.stderr)
        sys.exit(1)
    except TypeError as e:
        print(f"Error serializing data to JSON: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred while writing JSON to {output_filepath}: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Extract language comment tokens from a Helix TOML config file and output as JSON.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter
    )
    parser.add_argument(
        "input_file",
        type=Path,
        help="Path to the input TOML configuration file (e.g., languages.toml)"
    )
    parser.add_argument(
        "-o", "--output",
        type=Path,
        default="output.json",
        help="Path to the output JSON file."
    )

    args = parser.parse_args()

    extract_comment_tokens(args.input_file, args.output)
