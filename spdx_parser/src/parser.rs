// src/parser.rs
use crate::error::AppError;
use html_escape::{self, decode_html_entities};
use html_parser::{Dom, Node};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::atomic::{AtomicUsize, Ordering}; // Ensure decode_html_entities is imported

// Regex for text format parsing
static OPTIONAL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)<<beginOptional>>.*?<<endOptional>>").unwrap());

// Regex to find the HTML entity &apos;
static APOS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"&apos;").expect("Invalid regex pattern for &apos;"));

// Find var blocks
static VAR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"<<\s*var\s*;\s*name\s*=\s*"(?P<name>[^"]+)"\s*;\s*original\s*=\s*"(?P<original>.*?)"\s*;\s*match\s*=\s*"(?P<match>.*?)"\s*>>"#).unwrap()
});

// Counter for generating unique placeholder names for HTML variables
static PLACEHOLDER_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Parses the custom template text format and converts it to Handlebars.
pub fn parse_text_template(content: &str) -> Result<String, AppError> {
    log::debug!("Parsing text template");
    log::trace!("Original text content:\n{}", content);

    // |1| Remove optional blocks
    let without_optionals = OPTIONAL_REGEX.replace_all(content, "");
    log::trace!(
        "Text content after removing optionals:\n{}",
        without_optionals
    );

    // Check if the regex finds any matches at all
    if !VAR_REGEX.is_match(&without_optionals) {
        log::warn!(
            "VAR_REGEX did not find any matches in the text content after optional removal."
        );
    }

    // |2| Replace var blocks with Handlebars conditionals
    let handlebars_content =
        VAR_REGEX.replace_all(&without_optionals, |caps: &regex::Captures| {
            log::debug!("Text var regex matched! Processing replacement...");

            let name = &caps["name"];
            let original_raw = &caps["original"];
            // Decode HTML entities that might be in the original attribute
            let original_decoded = decode_html_entities(original_raw).to_string();
            let original_trimmed = original_decoded.trim();


            // Basic unescaping for Handlebars else block
            let original_content = original_trimmed.replace("{{", "\\{{").replace("}}", "\\}}");

            log::trace!(
                "Text var details: name='{}', original_raw='{}', original_decoded='{}', final_original='{}'",
                name,
                original_raw,
                original_decoded,
                original_content
            );

            // Format as Handlebars conditional with else block
            let replacement = format!(
                "{{{{#if {name}}}}}{{{{{name}}}}}{{{{else}}}}{original_content}{{{{/if}}}}",
                name = name,
                original_content = original_content
            );
            log::trace!("Text var replacement string: {}", replacement);
            replacement 
        });

    log::debug!("Finished parsing text template");
    log::trace!("Final text handlebars content:\n{}", handlebars_content);
    Ok(handlebars_content.to_string())
}

/// Helper to recursively extract all text from a list of nodes.
/// Returns raw text content, potentially including HTML entities.
fn extract_text_content(nodes: &[Node]) -> String {
    let mut text = String::new();
    for node in nodes {
        match node {
            Node::Text(t) => text.push_str(t),
            Node::Element(el) => {
                // Add space between block elements for visuals
                if !text.is_empty()
                    && !text.ends_with(char::is_whitespace)
                    && matches!(
                        el.name.as_str(),
                        "p" | "div"
                            | "li"
                            | "br"
                            | "h1"
                            | "h2"
                            | "h3"
                            | "h4"
                            | "h5"
                            | "h6"
                            | "ul"
                            | "ol"
                    )
                {
                    text.push(' ');
                }
                text.push_str(&extract_text_content(&el.children));
                // Ensure space after block elements
                if !text.ends_with(char::is_whitespace)
                    && matches!(
                        el.name.as_str(),
                        "p" | "div"
                            | "li"
                            | "br"
                            | "h1"
                            | "h2"
                            | "h3"
                            | "h4"
                            | "h5"
                            | "h6"
                            | "ul"
                            | "ol"
                    )
                {
                    text.push(' ');
                }
            }
            Node::Comment(_) => {}
        }
    }
    text.trim().to_string()
}

// Markdown version
pub fn parse_html_to_markdown_handlebars_v2(html_content: &str) -> Result<String, AppError> {
    log::debug!("Parsing HTML template (v2 using html2md)");

    // |1| Pre-process HTML to replace nodes with markers
    let mut replacements: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let dom = Dom::parse(html_content)?;
    PLACEHOLDER_COUNTER.store(0, Ordering::SeqCst); // Reset counter

    let mut processed_html = String::with_capacity(html_content.len());
    process_and_mark_html_nodes(&dom.children, &mut processed_html, &mut replacements)?;
    log::trace!("Replacements map created: {:?}", replacements.keys());

    // |2| Convert modified HTML to Markdown
    log::trace!("Converting marked HTML to Markdown:\n{}", processed_html);
    let mut markdown = html2md::parse_html(&processed_html);
    log::trace!(
        "Initial Markdown conversion (may contain escaped markers):\n{}",
        markdown
    );

    // |3| Post-process Markdown to insert Handlebars
    log::info!(
        "Inserting Handlebars into Markdown for {} placeholders",
        replacements.len()
    );
    let mut sorted_markers: Vec<_> = replacements.keys().cloned().collect();
    sorted_markers.sort_by_key(|m| {
        m.trim_start_matches("@@PLACEHOLDER_")
            .trim_end_matches("@@")
            .parse::<usize>()
            .unwrap_or(0)
    });

    for marker in sorted_markers {
        let original_content_raw = replacements.get(&marker).unwrap();
        let original_content_decoded = decode_html_entities(original_content_raw).to_string();
        let original_escaped_for_hbs = original_content_decoded
            .replace("{{", "\\{{")
            .replace("}}", "\\}}");

        let count_str = marker
            .trim_start_matches("@@PLACEHOLDER_")
            .trim_end_matches("@@");
        let placeholder_name = format!("placeholder{}", count_str);

        let handlebars_block = format!(
            "{{{{#if {name}}}}}{{{{{name}}}}}{{{{else}}}}{original}{{{{/if}}}}",
            name = placeholder_name,
            original = original_escaped_for_hbs
        );

        // Escaped marker to search for
        let escaped_marker = marker.replace('_', "\\_");

        log::trace!("Attempting replacement:");
        log::trace!("  Marker (Original): '{}'", marker);
        log::trace!("  Marker (Escaped for Search): '{}'", escaped_marker); // Log the escaped version
        log::trace!("  Original Raw: '{}'", original_content_raw);
        log::trace!("  Original Decoded: '{}'", original_content_decoded);
        log::trace!("  Handlebars Block: {}", handlebars_block);

        let markdown_before = markdown.clone();

        markdown = markdown.replace(&escaped_marker, &handlebars_block);

        if markdown == markdown_before {
            // If escaped didn't work, maybe it wasn't escaped? Try original as fallback.
            log::warn!(
                "Escaped marker '{}' was NOT found. Trying original marker '{}'...",
                escaped_marker,
                marker
            );
            markdown = markdown.replace(&marker, &handlebars_block);
            if markdown == markdown_before {
                log::error!(
                    "Original marker '{}' was ALSO NOT found or replaced in the markdown content!",
                    marker
                );
            } else {
                log::info!(
                    "Original marker '{}' replaced successfully after escaped failed.",
                    marker
                );
            }
        } else {
            log::trace!("Escaped marker '{}' replaced successfully.", escaped_marker);
        }
    }

    let without_apos = APOS_REGEX.replace_all(markdown.as_str(), "'");

    log::debug!("Finished parsing HTML template to Markdown/Handlebars (v2)");
    log::trace!("Final HTML->Markdown content:\n{}", markdown);
    Ok(without_apos.into_owned())
}

/// Recursively processes HTML nodes, replacing replaceable ones with markers.
fn process_and_mark_html_nodes(
    nodes: &[Node],
    output_html: &mut String,
    replacements: &mut std::collections::HashMap<String, String>,
) -> Result<(), AppError> {
    for node in nodes {
        match node {
            Node::Text(text) => {
                output_html.push_str(&html_escape::encode_text(text));
            }
            Node::Element(element) => {
                let is_optional = element.classes.iter().any(|c| c == "optional-license-text");
                let is_replaceable = element
                    .classes
                    .iter()
                    .any(|c| c == "replaceable-license-text");

                if is_optional {
                    log::trace!("Skipping optional element: <{}>", element.name);
                    continue;
                }

                if is_replaceable {
                    let original_content = extract_text_content(&element.children);
                    let count = PLACEHOLDER_COUNTER.fetch_add(1, Ordering::SeqCst);
                    let marker = format!("@@PLACEHOLDER_{}@@", count);

                    log::trace!(
                        "HTML replaceable node found: Creating marker '{}' for content: '{}'",
                        marker,
                        original_content
                    );
                    output_html.push_str(&marker);
                    replacements.insert(marker, original_content);
                    continue;
                }

                // Reconstruct the HTML element
                output_html.push('<');
                output_html.push_str(&element.name);
                for (attr, value) in &element.attributes {
                    output_html.push(' ');
                    output_html.push_str(attr);
                    output_html.push_str("=\"");
                    output_html.push_str(&html_escape::encode_quoted_attribute(
                        value.as_deref().unwrap_or(""),
                    ));
                    output_html.push('"');
                }
                if !element.classes.is_empty() {
                    output_html.push_str(" class=\"");
                    output_html.push_str(&element.classes.join(" "));
                    output_html.push('"');
                }
                if let Some(id) = &element.id {
                    output_html.push_str(" id=\"");
                    output_html.push_str(&html_escape::encode_quoted_attribute(id));
                    output_html.push('"');
                }

                let is_self_closing = matches!(
                    element.name.to_lowercase().as_str(),
                    "br" | "hr" | "img" | "input" | "meta" | "link"
                );

                if is_self_closing {
                    output_html.push_str(" />");
                } else {
                    output_html.push('>');
                    process_and_mark_html_nodes(&element.children, output_html, replacements)?;
                    output_html.push_str("</");
                    output_html.push_str(&element.name);
                    output_html.push('>');
                }
            }
            Node::Comment(comment) => {
                output_html.push_str("<!--");
                output_html.push_str(&html_escape::encode_text(comment));
                output_html.push_str("-->");
            }
        }
    }
    Ok(())
}
