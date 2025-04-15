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
    print("Using 'tomllib' (standard library).")
except ImportError:
    try:
        import toml
        _parser = toml
        _decode_error = toml.TomlDecodeError
        print("Using 'toml' package.")
    except ImportError:
        print("Error: Required TOML parser not found.", file=sys.stderr)
        print("Please install 'tomllib' (standard in Python 3.11+) or 'toml' (`pip install toml`).", file=sys.stderr)
        sys.exit(1)
# --- End TOML Parser Selection ---

def extract_language_data(input_filepath: Path, output_filepath: Path):
    """
    Parses a TOML configuration file (like Helix's languages.toml), extracts
    language name, comment tokens (single-line and block), and simple string
    file types (ignoring complex entries like globs), and writes them to a
    JSON file.

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

    language_data_map = {}

    # Check if the 'language' key exists and is a list
    if 'language' in config and isinstance(config['language'], list):
        print(f"Found {len(config['language'])} language definitions. Processing...")
        for lang_config in config['language']:
            # Ensure the item is a dictionary and has a 'name'
            if not isinstance(lang_config, dict):
                print("Warning: Skipping non-dictionary item in 'language' list.", file=sys.stderr)
                continue
            lang_name = lang_config.get('name')
            if not lang_name:
                print("Warning: Skipping language definition without a 'name'.", file=sys.stderr)
                continue

            lang_data = {}

            # --- Extract Single-Line Comment Token ---
            comment_token_single = lang_config.get('comment-token')
            comment_tokens_list = lang_config.get('comment-tokens')

            raw_comment_token = None
            if comment_token_single is not None:
                raw_comment_token = comment_token_single
            elif comment_tokens_list is not None:
                 raw_comment_token = comment_tokens_list

            if isinstance(raw_comment_token, str):
                 lang_data['comment_token'] = raw_comment_token
            elif isinstance(raw_comment_token, list) and raw_comment_token:
                 lang_data['comment_token'] = raw_comment_token[0]
                 # Optionally store all: lang_data['comment_tokens_list'] = raw_comment_token

            # --- Extract Block Comment Tokens ---
            block_tokens = lang_config.get('block-comment-tokens')
            if block_tokens is not None:
                 lang_data['block_comment_tokens'] = block_tokens

            # --- Extract File Types --- ******************************** MODIFIED ********************************
            file_types_raw = lang_config.get('file-types') # Use a different var name initially
            if file_types_raw is not None:
                if isinstance(file_types_raw, list):
                    # Filter the list to keep only string elements using list comprehension
                    string_file_types = [ft for ft in file_types_raw if isinstance(ft, str)]

                    # Check if any non-string types were skipped and issue a warning
                    original_len = len(file_types_raw)
                    filtered_len = len(string_file_types)
                    if original_len > filtered_len:
                         skipped_count = original_len - filtered_len
                         print(f"Warning: Skipped {skipped_count} non-string entr{'y' if skipped_count == 1 else 'ies'} "
                               f"(e.g., globs) in 'file-types' for language '{lang_name}'. Keeping only string types.", file=sys.stderr)

                    # Store the filtered list (it might be empty if no strings were found)
                    # Only add the key if there were string file types originally or if you always want the key present
                    # Let's add it even if empty, to show 'file-types' was processed.
                    lang_data['file_types'] = string_file_types

                else:
                    # Handle the case where file-types exists but isn't a list
                    print(f"Warning: 'file-types' for language '{lang_name}' is not a list (found type {type(file_types_raw)}). Skipping file-types for this language.", file=sys.stderr)
            # ***********************************************************************************************

            # Only add the language if *any* data was found for it
            if lang_data:
                language_data_map[lang_name] = lang_data
            else:
                print(f"Note: No comment tokens or string file types found for language '{lang_name}'. Not adding to output.", file=sys.stderr)
    else:
        print("Warning: No '[[language]]' sections found or 'language' key is not a list in the configuration.", file=sys.stderr)


    # Write the extracted data to the JSON file
    print(f"Writing extracted language data to: {output_filepath}")
    try:
        output_filepath.parent.mkdir(parents=True, exist_ok=True)
        with open(output_filepath, 'w', encoding='utf-8') as f:
            json.dump(language_data_map, f, indent=4, ensure_ascii=False)
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
        description="Extract language comment tokens and simple string file types (skipping globs/complex entries) from a Helix TOML config file and output as JSON.",
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
        default=Path("language_data.json"),
        help="Path to the output JSON file."
    )

    args = parser.parse_args()

    extract_language_data(args.input_file, args.output)
