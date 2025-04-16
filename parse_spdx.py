#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import xml.etree.ElementTree as ET
import re # Used for cleaning whitespace and finding placeholders
import textwrap # Used for formatting notes in YAML and item indentation

# --- Configuration ---
XML_EXTENSION = ".xml"
MARKDOWN_EXTENSION = ".md"
CURRENT_DIRECTORY = os.getcwd() # Get the directory where the script is run

# Namespace used in the SPDX XML files
# ElementTree requires this for finding tags correctly
NAMESPACE = {'spdx': 'http://www.spdx.org/license'}

# --- Helper Functions ---

def clean_text(text):
    """Removes leading/trailing whitespace and collapses internal whitespace."""
    if text is None:
        return ""
    text = re.sub(r'\s+', ' ', text)
    return text.strip()

def get_element_text_content(element, process_placeholders=False):
    """
    Extracts text content from an element and its children, handling <br>.
    Collapses insignificant whitespace within text nodes to allow proper text flow.
    """
    text_parts = []

    if element.text:
        processed_text = re.sub(r'\s+', ' ', element.text).strip()
        if process_placeholders:
            processed_text = re.sub(r'&lt;([^>]+?)&gt;', r'{{\1}}', processed_text)
            processed_text = re.sub(r'<([^>]+?)>', r'{{\1}}', processed_text)
        if processed_text:
            text_parts.append(processed_text)

    for child in element:
        tag_name = child.tag.lower().replace(f"{{{NAMESPACE['spdx']}}}", "")
        if tag_name == 'br':
            if text_parts and not text_parts[-1].endswith((' ', '\n')):
                 text_parts.append(' ')
            text_parts.append("  \n")
        elif element.tag.replace(f"{{{NAMESPACE['spdx']}}}", "") == 'optional':
             continue
        else:
             child_content = get_element_text_content(child, process_placeholders)
             if child_content:
                 if text_parts and not text_parts[-1].endswith((' ', '\n')):
                     text_parts.append(' ')
                 text_parts.append(child_content)

        if child.tail:
            processed_tail = re.sub(r'\s+', ' ', child.tail).strip()
            if process_placeholders:
                 processed_tail = re.sub(r'&lt;([^>]+?)&gt;', r'{{\1}}', processed_tail)
                 processed_tail = re.sub(r'<([^>]+?)>', r'{{\1}}', processed_tail)
            if processed_tail:
                 if text_parts and not text_parts[-1].endswith((' ', '\n')):
                     text_parts.append(' ')
                 text_parts.append(processed_tail)

    full_text = ''.join(text_parts).strip()
    full_text = re.sub(r'(?<! {2})\s{2,}', ' ', full_text)
    return full_text


def format_yaml_value(value):
    """Formats a Python value for YAML output, handling strings, lists, booleans."""
    if isinstance(value, list):
        if not value:
            return "[]"
        items = [f"  - \"{str(item).replace('\"', '\\\"')}\"" for item in value]
        return "\n" + "\n".join(items)
    elif isinstance(value, str):
        if '\n' in value or len(value) > 60:
            indented_value = textwrap.indent(value, '  ')
            return f"|\n{indented_value}"
        elif ': ' in value or \
             value.lower() in ['true', 'false', 'null', 'yes', 'no', 'on', 'off'] or \
             value.startswith(('*', '-', '&', '!', '[', '{', '>', '|', '%', '@', '`')):
             return f"\"{value.replace('\"', '\\\"')}\""
        else:
            return value
    elif isinstance(value, bool):
        return str(value).lower()
    else:
        return str(value)

def process_spdx_element(element, indent_level=0):
    """
    Recursively processes SPDX XML elements (within <text>) and converts
    them to Markdown. Skips content within <optional> tags.
    Handles nested lists and indentation correctly. Uses '-' for default bullets.
    Ensures blank lines between list items.
    """
    output = ""
    tag_name = element.tag.replace(f"{{{NAMESPACE['spdx']}}}", "")
    indent = "  " * indent_level

    if tag_name == 'optional':
        return ""

    elif tag_name == 'p':
        text = get_element_text_content(element, process_placeholders=True)
        if text:
            # Indent using textwrap, ensure it ends with double newline for paragraph space
            output = textwrap.indent(text.strip(), indent, predicate=lambda line: True)
            output = output.rstrip() + "\n\n"

    elif tag_name == 'titleText':
        text = get_element_text_content(element, process_placeholders=True)
        if text:
             output = f"# {text.strip()}\n\n"

    elif tag_name == 'copyrightText':
        text = get_element_text_content(element, process_placeholders=True)
        if text:
             output = f"**{text.strip()}**\n\n"

    elif tag_name == 'list':
        list_items_output = []
        for item_element in element.findall('spdx:item', NAMESPACE):
            list_items_output.append(process_spdx_element(item_element, indent_level))
        # Join item markdown directly. Item processing now adds trailing \n\n.
        output = "".join(list_items_output)
        # No extra newline needed here; the last item's \n\n handles list block spacing.

    elif tag_name == 'item':
        bullet_element = element.find('spdx:bullet', NAMESPACE)
        has_bullet = bullet_element is not None and bullet_element.text and bullet_element.text.strip()
        bullet_text = clean_text(bullet_element.text) if has_bullet else "-"
        bullet_marker = f"{bullet_text} "

        # --- Collect and Process Item Content ---
        item_initial_text = ""
        if has_bullet and bullet_element.tail:
             item_initial_text = clean_text(bullet_element.tail)
        elif not has_bullet and element.text:
             item_initial_text = clean_text(element.text)
        item_initial_text = re.sub(r'&lt;([^>]+?)&gt;', r'{{\1}}', item_initial_text)
        item_initial_text = re.sub(r'<([^>]+?)>', r'{{\1}}', item_initial_text)

        child_content_parts = []
        for child in element:
            child_tag_name = child.tag.replace(f"{{{NAMESPACE['spdx']}}}", "")
            if child_tag_name == 'bullet':
                continue
            # Process child recursively, passing indent + 1
            processed_child = process_spdx_element(child, indent_level + 1)
            if processed_child: # Check if not empty
                # Append the raw result (already has indentation and spacing)
                child_content_parts.append(processed_child)

        # --- Combine and Format Item ---
        children_block = "".join(child_content_parts).rstrip() # Join & remove trailing space only

        # Start the output line for the item
        item_output_lines = []
        first_line = f"{indent}{bullet_marker}{item_initial_text}"
        item_output_lines.append(first_line.rstrip()) # Add first line, remove trailing space

        # Add children block if it exists
        if children_block:
            # Ensure children start on a new line relative to the first line text
            item_output_lines.append(children_block)

        # Join the lines for this item's content
        output = "\n".join(item_output_lines).rstrip()

        # *** Add blank line (two newlines) AFTER the item for spacing ***
        output += "\n\n"

    elif tag_name == 'standardLicenseHeader':
        header_content = get_element_text_content(element, process_placeholders=True)
        if header_content:
             header_content = header_content.strip()
             # Code block itself provides spacing, but ensure trailing \n\n after it
             code_block = f"```\n{header_content}\n```"
             output = textwrap.indent(code_block, indent, predicate=lambda line: True) + "\n\n"

    elif tag_name == 'alt':
         pass # Processed within parent via get_element_text_content

    elif tag_name == 'br':
        output = "  \n" # Output hard break

    else:
        # Fallback: Treat as paragraph
        text_fallback = get_element_text_content(element, process_placeholders=True)
        if text_fallback:
             output = textwrap.indent(text_fallback.strip(), indent, predicate=lambda line: True) + "\n\n"

    return output


def xml_to_markdown(xml_filepath):
    """
    Parses an SPDX XML file and converts its content to Markdown format
    with YAML front matter for metadata. Skips content within <optional> tags.
    Handles nested lists correctly. Uses '-' for default bullets.
    Ensures blank lines between list items.
    """
    try:
        tree = ET.parse(xml_filepath)
        root = tree.getroot()
    except ET.ParseError as e:
        print(f"  [Error] Failed to parse XML file: {xml_filepath}. Reason: {e}")
        return None
    except FileNotFoundError:
        print(f"  [Error] XML file not found: {xml_filepath}")
        return None

    license_element = root.find('spdx:license', NAMESPACE)
    if license_element is None:
        print(f"  [Warning] No <license> tag found in: {xml_filepath}")
        license_element = root.find('license') # Fallback
        if license_element is None:
             print(f"  [Error] Still couldn't find <license> tag. Skipping file.")
             return None

    # --- Extract Metadata (Unchanged) ---
    metadata = {}
    metadata['spdxID'] = license_element.get('licenseId', 'N/A')
    metadata['name'] = license_element.get('name', 'N/A')
    osi_approved_str = license_element.get('isOsiApproved', 'unknown').lower()
    if osi_approved_str == 'true': metadata['osiApproved'] = "true"
    elif osi_approved_str == 'false': metadata['osiApproved'] = "false"
    else: metadata['osiApproved'] = 'unknown'
    metadata['listVersionAdded'] = license_element.get('listVersionAdded', 'Unknown')
    cross_refs_list = []
    cross_refs_element = license_element.find('spdx:crossRefs', NAMESPACE)
    if cross_refs_element is not None:
        refs = cross_refs_element.findall('spdx:crossRef', NAMESPACE)
        for ref in refs:
            if ref.text and ref.text.strip(): cross_refs_list.append(ref.text.strip())
    if cross_refs_list: metadata['crossRefs'] = cross_refs_list
    notes_element = license_element.find('spdx:notes', NAMESPACE)
    if notes_element is not None:
        notes_text_parts = []
        if notes_element.text: notes_text_parts.append(notes_element.text.strip())
        for child in notes_element:
            if child.text: notes_text_parts.append(child.text.strip())
            if child.tail: notes_text_parts.append(child.tail.strip())
        notes_text = '\n\n'.join(filter(None, notes_text_parts)).strip()
        notes_text = re.sub(r'\s*\n\s*', '\n', notes_text)
        if notes_text: metadata['notes'] = notes_text

    # --- Format Metadata as YAML Front Matter (Unchanged) ---
    yaml_lines = ["---"]
    for key, value in metadata.items():
        yaml_lines.append(f"{key}: {format_yaml_value(value)}")
    yaml_lines.append("---")
    yaml_front_matter = "\n".join(yaml_lines)

    # --- Extract and Process License Text Body ---
    markdown_body_parts = []
    text_element = license_element.find('spdx:text', NAMESPACE)
    if text_element is not None:
        for child_element in text_element:
             processed_text = process_spdx_element(child_element, indent_level=0)
             if processed_text:
                 markdown_body_parts.append(processed_text)
    else:
        markdown_body_parts.append("*(License text not found in XML)*\n")

    # --- Combine Body Parts ---
    # Join directly, trailing \n\n from elements handles spacing
    markdown_body = "".join(markdown_body_parts).strip()
    # Final cleanup of excess blank lines
    markdown_body = re.sub(r'\n{3,}', '\n\n', markdown_body)

    # --- Final Placeholder Post-Processing (Safety Net - Unchanged) ---
    markdown_body = re.sub(r'&lt;([^>]+?)&gt;', r'{{\1}}', markdown_body)
    markdown_body = re.sub(r'<([^>]+?)>', r'{{\1}}', markdown_body)

    # --- Combine YAML and Body ---
    final_markdown = yaml_front_matter + "\n\n" + markdown_body

    return final_markdown


# --- Main Execution (Unchanged) ---
if __name__ == "__main__":
    print(f"Starting SPDX XML to Markdown conversion in directory: {CURRENT_DIRECTORY}")
    converted_count = 0
    error_count = 0

    for filename in os.listdir(CURRENT_DIRECTORY):
        if filename.lower().endswith(XML_EXTENSION):
            xml_filepath = os.path.join(CURRENT_DIRECTORY, filename)
            print(f"\nProcessing file: {filename}...")
            base_name = os.path.splitext(filename)[0]
            md_filename = base_name + MARKDOWN_EXTENSION
            md_filepath = os.path.join(CURRENT_DIRECTORY, md_filename)

            markdown_content = xml_to_markdown(xml_filepath)

            if markdown_content is not None:
                try:
                    # Final cleanup for writing: ensure single trailing newline
                    markdown_content = markdown_content.rstrip() + '\n'
                    with open(md_filepath, 'w', encoding='utf-8') as f:
                        f.write(markdown_content)
                    print(f"  [Success] Converted '{filename}' to '{md_filename}'")
                    converted_count += 1
                except IOError as e:
                    print(f"  [Error] Could not write Markdown file: {md_filepath}. Reason: {e}")
                    error_count += 1
            else:
                error_count += 1

    print(f"\nConversion finished.")
    print(f"Successfully converted: {converted_count} file(s)")
    print(f"Encountered errors: {error_count} file(s)")
