#!/usr/bin/env uv run
# -*- coding: utf-8 -*-

# --- Dependencies ---
# Standard Library:
import os
import xml.etree.ElementTree as ET
import re
import textwrap
import sys

# Third-Party:
# Install using: pip install click halo
try:
    import click
    from halo import Halo
except ImportError:
    print("Error: Required libraries 'click' and 'halo' are not installed.", file=sys.stderr)
    print("Please install them using: pip install click halo", file=sys.stderr)
    sys.exit(1)

# --- Configuration ---
XML_EXTENSION = ".xml"
MARKDOWN_EXTENSION = ".md"
TEXT_EXTENSION = ".txt"

# Namespace used in the SPDX XML files
NAMESPACE = {'spdx': 'http://www.spdx.org/license'}

# --- Helper Functions ---

def clean_text(text):
    """Removes leading/trailing whitespace and collapses internal whitespace."""
    if text is None:
        return ""
    # Replace various whitespace chars with a single space, then strip
    text = re.sub(r'\s+', ' ', str(text))
    return text.strip()

def get_element_text_content(element, output_format='markdown'):
    """
    Extracts text content from an element and its children.
    Handles <br> differently based on output format.
    Collapses insignificant whitespace.
    Processes placeholders like <var> into {{var}} for markdown.
    Strips placeholders for text output.
    """
    text_parts = []
    process_placeholders = (output_format == 'markdown')

    if element.text:
        processed_text = re.sub(r'\s+', ' ', element.text).strip()
        if process_placeholders:
            processed_text = re.sub(r'&lt;([^>]+?)&gt;', r'{{\1}}', processed_text)
            processed_text = re.sub(r'<([^>]+?)>', r'{{\1}}', processed_text)
        else:
            # Strip placeholders for text output
            processed_text = re.sub(r'&lt;[^>]+?&gt;', '', processed_text)
            processed_text = re.sub(r'<[^>]+?>', '', processed_text)
        if processed_text:
            text_parts.append(processed_text)

    for child in element:
        tag_name = child.tag.lower().replace(f"{{{NAMESPACE['spdx']}}}", "")

        # Skip optional content entirely
        if tag_name == 'optional':
            if child.tail: # Still need to process tail of the skipped element
                 processed_tail = re.sub(r'\s+', ' ', child.tail).strip()
                 if process_placeholders:
                     processed_tail = re.sub(r'&lt;([^>]+?)&gt;', r'{{\1}}', processed_tail)
                     processed_tail = re.sub(r'<([^>]+?)>', r'{{\1}}', processed_tail)
                 else:
                     processed_tail = re.sub(r'&lt;[^>]+?&gt;', '', processed_tail)
                     processed_tail = re.sub(r'<[^>]+?>', '', processed_tail)

                 if processed_tail:
                    if text_parts and not text_parts[-1].endswith((' ', '\n')):
                         text_parts.append(' ')
                    text_parts.append(processed_tail)
            continue # Skip processing children of <optional>

        # Handle line breaks based on format
        if tag_name == 'br':
            # Ensure space before break if needed
            if text_parts and not text_parts[-1].endswith((' ', '\n')):
                text_parts.append(' ')
            # Markdown hard break vs Text newline
            text_parts.append("  \n" if output_format == 'markdown' else "\n")
        else:
            # Recursively get child content
            child_content = get_element_text_content(child, output_format)
            if child_content:
                # Ensure space between parent text/previous child and this child if needed
                if text_parts and not text_parts[-1].endswith((' ', '\n')):
                    text_parts.append(' ')
                text_parts.append(child_content)

        # Process text following the child element (tail)
        if child.tail:
            processed_tail = re.sub(r'\s+', ' ', child.tail).strip()
            if process_placeholders:
                processed_tail = re.sub(r'&lt;([^>]+?)&gt;', r'{{\1}}', processed_tail)
                processed_tail = re.sub(r'<([^>]+?)>', r'{{\1}}', processed_tail)
            else:
                processed_tail = re.sub(r'&lt;[^>]+?&gt;', '', processed_tail)
                processed_tail = re.sub(r'<[^>]+?>', '', processed_tail)

            if processed_tail:
                # Ensure space before tail if needed
                if text_parts and not text_parts[-1].endswith((' ', '\n')):
                    text_parts.append(' ')
                text_parts.append(processed_tail)

    full_text = ''.join(text_parts).strip()
    # Collapse multiple spaces unless it's the double space for markdown hard break
    if output_format == 'markdown':
        full_text = re.sub(r'(?<! {2})\s{2,}', ' ', full_text)
    else:
        full_text = re.sub(r'\s{2,}', ' ', full_text) # Collapse all multiple spaces for text
        full_text = re.sub(r' (\n)', r'\1', full_text) # Remove space before newline
    return full_text


def format_yaml_value(value):
    """Formats a Python value for YAML output, handling strings, lists, booleans."""
    if isinstance(value, list):
        if not value:
            return "[]"
        # Use double quotes for list items for consistency
        items = [f'  - "{str(item).replace("\"", "\\\"")}"' for item in value]
        return "\n" + "\n".join(items)
    elif isinstance(value, str):
        # Check for multiline, special chars, length, or ':' to decide quoting/blocking
        if '\n' in value:
             # Indent block scalar correctly
            indented_value = textwrap.indent(value, '  ')
            return f"|\n{indented_value}"
        elif ':' in value or value.startswith(('*', '-', '&', '!', '[', '{', '>', '|', '%', '@', '`')) or \
             value.lower() in ['true', 'false', 'null', 'yes', 'no', 'on', 'off'] or \
             len(value) > 70: # Arbitrary length limit for quoting
            # Use double quotes for safety
            return f'"{value.replace("\"", "\\\"")}"'
        else:
            # Simple string
            return value
    elif isinstance(value, bool):
        return str(value).lower()
    else:
        # Numbers or other types
        return str(value)

# --- Core Processing Logic ---

def process_spdx_element(element, output_format, indent_level=0):
    """
    Recursively processes SPDX XML elements (within <text>) and converts
    them to the specified format (Markdown or Text). Skips <optional>.
    """
    output = ""
    tag_name = element.tag.replace(f"{{{NAMESPACE['spdx']}}}", "")
    indent = "  " * indent_level  # Using 2 spaces for indentation

    if tag_name == 'optional':
        return "" # Skip optional content

    # --- Format Specific Handling ---
    if output_format == 'markdown':
        if tag_name == 'p':
            text = get_element_text_content(element, output_format)
            if text:
                # Apply indent only to the first line for paragraphs after lists
                # Use textwrap for consistent indentation within the paragraph
                wrapped_text = textwrap.fill(text, width=100, initial_indent=indent, subsequent_indent=indent, replace_whitespace=False)
                output = wrapped_text + "\n\n" # Ensure paragraph spacing
        elif tag_name == 'titleText':
            text = get_element_text_content(element, output_format)
            if text:
                output = f"{indent}# {text.strip()}\n\n"
        elif tag_name == 'copyrightText':
            text = get_element_text_content(element, output_format)
            if text:
                 # Use emphasis (bold) for copyright in Markdown
                output = f"{indent}**{text.strip()}**\n\n"
        elif tag_name == 'list':
            list_items_output = []
            for item_element in element.findall('spdx:item', NAMESPACE):
                 # Recursively process items, incrementing indent for nested lists
                list_items_output.append(process_spdx_element(item_element, output_format, indent_level))
            output = "".join(list_items_output) # Items manage their own spacing
        elif tag_name == 'item':
            bullet_element = element.find('spdx:bullet', NAMESPACE)
            bullet_text = clean_text(bullet_element.text) if bullet_element is not None and bullet_element.text else "-"
            bullet_marker = f"{bullet_text} "

            # Get text content excluding the bullet itself
            item_content_text = get_element_text_content(element, output_format)
            # Remove the bullet text from the start if it's present (it gets duplicated otherwise)
            if item_content_text.startswith(bullet_text):
                 item_content_text = item_content_text[len(bullet_text):].lstrip()

            if item_content_text:
                 # Indent the first line with bullet, subsequent lines align with text
                 first_line_indent = indent + bullet_marker
                 subsequent_indent = indent + " " * len(bullet_marker)
                 wrapped_item = textwrap.fill(item_content_text, width=100,
                                             initial_indent=first_line_indent,
                                             subsequent_indent=subsequent_indent,
                                             replace_whitespace=False, # Preserve hard breaks from <br>
                                             break_long_words=False)
                 output = wrapped_item.rstrip() + "\n\n" # Ensure blank line after item
            else: # Handle items that might *only* contain nested lists
                 output = "" # Let nested lists handle their output

            # Process nested children (like sub-lists) within the item
            child_content_parts = []
            for child in element:
                 child_tag = child.tag.replace(f"{{{NAMESPACE['spdx']}}}", "")
                 if child_tag != 'bullet': # Don't re-process bullet
                     # Indent nested content further
                     processed_child = process_spdx_element(child, output_format, indent_level + 1)
                     if processed_child:
                         child_content_parts.append(processed_child)
            output += "".join(child_content_parts) # Append nested content

        elif tag_name == 'standardLicenseHeader':
            header_content = get_element_text_content(element, output_format).strip()
            if header_content:
                 # Format as a code block in Markdown
                 code_block = f"```\n{header_content}\n```"
                 output = textwrap.indent(code_block, indent, predicate=lambda line: True) + "\n\n"
        elif tag_name == 'br':
             # Handled within get_element_text_content, but need placeholder if element is *only* <br>
             pass
        else:
            # Fallback: Treat unknown tags like paragraphs
            text_fallback = get_element_text_content(element, output_format)
            if text_fallback:
                 wrapped_text = textwrap.fill(text_fallback, width=100, initial_indent=indent, subsequent_indent=indent, replace_whitespace=False)
                 output = wrapped_text + "\n\n"

    elif output_format == 'text':
        if tag_name in ['p', 'titleText', 'copyrightText']:
            text = get_element_text_content(element, output_format)
            if text:
                # Simple text block with standard indentation
                wrapped_text = textwrap.fill(text, width=100, initial_indent=indent, subsequent_indent=indent, replace_whitespace=False)
                output = wrapped_text + "\n\n" # Double newline for paragraph separation
        elif tag_name == 'list':
             list_items_output = []
             for item_element in element.findall('spdx:item', NAMESPACE):
                 # Process items with the same indentation level for text lists
                 list_items_output.append(process_spdx_element(item_element, output_format, indent_level))
             output = "".join(list_items_output) # Join items
        elif tag_name == 'item':
            bullet_element = element.find('spdx:bullet', NAMESPACE)
            # Use a simple dash for text list items, ignore custom bullets
            bullet_marker = "- "

            item_content_text = get_element_text_content(element, output_format)
             # Remove the original bullet text if present
            if bullet_element is not None and bullet_element.text and item_content_text.startswith(clean_text(bullet_element.text)):
                 item_content_text = item_content_text[len(clean_text(bullet_element.text)):].lstrip()

            if item_content_text:
                first_line_indent = indent + bullet_marker
                subsequent_indent = indent + "  " # Indent subsequent lines of item
                wrapped_item = textwrap.fill(item_content_text, width=100,
                                             initial_indent=first_line_indent,
                                             subsequent_indent=subsequent_indent,
                                             replace_whitespace=False) # Preserve newlines from <br>
                output = wrapped_item.rstrip() + "\n" # Single newline after text item
            else:
                output = "" # Handle items with only nested content

            # Process nested children (like sub-lists) within the item
            child_content_parts = []
            for child in element:
                 child_tag = child.tag.replace(f"{{{NAMESPACE['spdx']}}}", "")
                 if child_tag != 'bullet':
                     # Increase indent for nested structures in text
                     processed_child = process_spdx_element(child, output_format, indent_level + 1)
                     if processed_child:
                         child_content_parts.append(processed_child)
            # Indent the entire nested block
            output += "".join(child_content_parts)

        elif tag_name == 'standardLicenseHeader':
            header_content = get_element_text_content(element, output_format).strip()
            if header_content:
                 # Just output the text, possibly with separator lines
                 output = f"{indent}--- LICENSE HEADER START ---\n"
                 output += textwrap.indent(header_content, indent, predicate=lambda line: True)
                 output += f"\n{indent}--- LICENSE HEADER END ---\n\n"
        elif tag_name == 'br':
             # Handled within get_element_text_content
             pass
        else:
            # Fallback for text: Treat like paragraphs
            text_fallback = get_element_text_content(element, output_format)
            if text_fallback:
                 wrapped_text = textwrap.fill(text_fallback, width=100, initial_indent=indent, subsequent_indent=indent, replace_whitespace=False)
                 output = wrapped_text + "\n\n"

    return output


def extract_metadata(license_element):
    """Extracts metadata from the license element."""
    metadata = {}
    try:
        metadata['spdxID'] = license_element.get('licenseId', 'N/A')
        metadata['name'] = license_element.get('name', 'N/A')

        osi_approved_str = license_element.get('isOsiApproved', 'unknown').lower()
        if osi_approved_str == 'true': metadata['osiApproved'] = True
        elif osi_approved_str == 'false': metadata['osiApproved'] = False
        else: metadata['osiApproved'] = 'unknown' # Keep as string for YAML clarity

        metadata['listVersionAdded'] = license_element.get('listVersionAdded', 'Unknown')

        # Cross References
        cross_refs_list = []
        cross_refs_element = license_element.find('spdx:crossRefs', NAMESPACE)
        if cross_refs_element is not None:
            refs = cross_refs_element.findall('spdx:crossRef', NAMESPACE)
            cross_refs_list.extend(clean_text(ref.text) for ref in refs if ref.text)
        if cross_refs_list: metadata['crossRefs'] = cross_refs_list

        # Notes
        notes_element = license_element.find('spdx:notes', NAMESPACE)
        if notes_element is not None:
            # Use get_element_text_content to handle potential formatting within notes
            notes_text = get_element_text_content(notes_element, 'text') # Get notes as plain text
            if notes_text: metadata['notes'] = notes_text.strip()

    except Exception as e:
        # Log metadata extraction error, but don't halt processing
        click.echo(click.style(f"  [Warning] Error extracting metadata: {e}", fg='yellow'), err=True)
    return metadata

def generate_output(xml_filepath, output_format):
    """
    Parses SPDX XML and generates content in the specified format.
    Returns the generated content string or None on critical error.
    Raises exceptions for file/parsing issues.
    """
    tree = ET.parse(xml_filepath) # Raises ParseError, FileNotFoundError
    root = tree.getroot()

    license_element = root.find('spdx:license', NAMESPACE)
    if license_element is None:
        # Try finding without namespace as fallback
        license_element = root.find('license')
        if license_element is None:
            raise ValueError("Could not find the required <license> tag in the XML.")

    # --- Extract Body ---
    body_parts = []
    text_element = license_element.find('spdx:text', NAMESPACE)
    if text_element is None:
         text_element = license_element.find('text') # Fallback

    if text_element is not None:
        # Process direct text content of <text> if any (unlikely but possible)
        if text_element.text and text_element.text.strip():
             body_parts.append(clean_text(text_element.text)+ "\n\n")

        # Process children of <text>
        for child_element in text_element:
            processed_text = process_spdx_element(child_element, output_format, indent_level=0)
            if processed_text:
                 # Ensure consistent spacing between processed elements
                 body_parts.append(processed_text.rstrip() + "\n\n")
             # Capture tail text after child elements within <text>
            if child_element.tail and child_element.tail.strip():
                body_parts.append(clean_text(child_element.tail)+ "\n\n")

    else:
        if output_format == 'markdown':
            body_parts.append("*(License text body not found in XML)*\n")
        else:
            body_parts.append("(License text body not found in XML)\n")


    # --- Combine Body Parts ---
    output_body = "".join(body_parts).strip()
    # Final cleanup of excess blank lines
    output_body = re.sub(r'\n{3,}', '\n\n', output_body)

    # --- Add Metadata/YAML (Markdown only) ---
    if output_format == 'markdown':
        metadata = extract_metadata(license_element)
        yaml_lines = ["---"]
        for key, value in metadata.items():
            yaml_lines.append(f"{key}: {format_yaml_value(value)}")
        yaml_lines.append("---")
        yaml_front_matter = "\n".join(yaml_lines)
        final_output = yaml_front_matter + "\n\n" + output_body
    else:
        # For text output, just return the body
        final_output = output_body

    return final_output.strip() + '\n' # Ensure single trailing newline

# --- Click Command ---

@click.command()
@click.option('--input-dir', '-i',
              type=click.Path(exists=True, file_okay=False, dir_okay=True, readable=True),
              default='.', show_default=True,
              help='Directory containing the SPDX XML files.')
@click.option('--output-dir', '-o',
              type=click.Path(file_okay=False, dir_okay=True, writable=True, resolve_path=True),
              default='.', show_default=True,
              help='Directory to save the output files.')
@click.option('--output-type', '-t',
              type=click.Choice(['markdown', 'text'], case_sensitive=False),
              default='markdown', show_default=True,
              help='Output format.')
def main(input_dir, output_dir, output_type):
    """
    Converts SPDX XML license files to Markdown or plain Text format.

    Reads all .xml files from the INPUT_DIR, processes them according
    to the specified OUTPUT_TYPE, and saves the results in OUTPUT_DIR.
    """
    click.echo(f"Starting SPDX XML conversion")
    click.echo(f"Input directory: {os.path.abspath(input_dir)}")
    click.echo(f"Output directory: {output_dir}")
    click.echo(f"Output format: {output_type}")

    # Ensure output directory exists
    try:
        os.makedirs(output_dir, exist_ok=True)
    except OSError as e:
        click.echo(click.style(f"Error: Could not create output directory '{output_dir}': {e}", fg='red'), err=True)
        sys.exit(1)

    # Find XML files
    try:
        xml_files = [f for f in os.listdir(input_dir) if f.lower().endswith(XML_EXTENSION)]
    except OSError as e:
        click.echo(click.style(f"Error: Could not read input directory '{input_dir}': {e}", fg='red'), err=True)
        sys.exit(1)

    if not xml_files:
        click.echo(click.style(f"No XML files found in '{input_dir}'. Nothing to process.", fg='yellow'))
        sys.exit(0)

    total_files = len(xml_files)
    converted_count = 0
    error_count = 0

    spinner = Halo(text='Initializing...', spinner='dots')
    spinner.start()

    for i, filename in enumerate(xml_files, 1):
        current_file_number = i
        spinner.text = f'[{current_file_number}/{total_files}] Processing: {filename}'
        xml_filepath = os.path.join(input_dir, filename)
        base_name = os.path.splitext(filename)[0]

        if output_type == 'markdown':
            out_extension = MARKDOWN_EXTENSION
            out_filename = base_name + out_extension
        else: # 'text'
            out_extension = TEXT_EXTENSION
            out_filename = base_name + out_extension

        out_filepath = os.path.join(output_dir, out_filename)

        try:
            # --- Generate Content ---
            output_content = generate_output(xml_filepath, output_type)

            # --- Write Output ---
            with open(out_filepath, 'w', encoding='utf-8') as f:
                f.write(output_content)

            spinner.succeed(f'[{current_file_number}/{total_files}] Converted: {filename} -> {out_filename}')
            converted_count += 1

        except FileNotFoundError:
            spinner.fail(f"[{current_file_number}/{total_files}] Error: Input file not found (should not happen): {filename}")
            error_count += 1
        except ET.ParseError as e:
            spinner.fail(f"[{current_file_number}/{total_files}] Error: Failed to parse XML: {filename}. Reason: {e}")
            error_count += 1
        except ValueError as e: # Custom error for missing <license> tag
            spinner.fail(f"[{current_file_number}/{total_files}] Error: Invalid XML structure: {filename}. Reason: {e}")
            error_count += 1
        except IOError as e:
            spinner.fail(f"[{current_file_number}/{total_files}] Error: Could not write output file: {out_filename}. Reason: {e}")
            error_count += 1
        except PermissionError as e:
             spinner.fail(f"[{current_file_number}/{total_files}] Error: Permission denied for file: {filename} or {out_filename}. Reason: {e}")
             error_count += 1
        except Exception as e: # Catch-all for unexpected errors during processing
            spinner.fail(f"[{current_file_number}/{total_files}] Error: Unexpected error processing {filename}: {e}")
            import traceback
            traceback.print_exc(file=sys.stderr) # Print traceback for debugging
            error_count += 1

        # Reset spinner text for the next file
        spinner.text = f'[{current_file_number}/{total_files}] Moving to next file...'


    spinner.stop()
    click.echo("\n--- Conversion Summary ---")
    click.echo(click.style(f"Successfully converted: {converted_count} file(s)", fg='green'))
    if error_count > 0:
        click.echo(click.style(f"Encountered errors: {error_count} file(s)", fg='red'))
    else:
        click.echo("Encountered errors: 0 file(s)")
    click.echo("--------------------------")

    if error_count > 0:
        sys.exit(1) # Exit with error code if any files failed

# --- Main Execution Guard ---
if __name__ == "__main__":
    main()
