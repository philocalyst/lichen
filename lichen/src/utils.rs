//! # Utility Functions
//!
//! General helper functions for file processing, text formatting, etc.

use crate::config::{Authors, Config};
use crate::error::LichenError;
use crate::models::CommentToken;
use handlebars::{Handlebars, RenderError};
use jiff::civil::Date;
use log::{debug, error, info, trace, warn};
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::fs::{self};
use std::path::MAIN_SEPARATOR;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use walkdir::{self, WalkDir}; // Use tokio's async fs

// Embed comment tokens at build-time
const COMMENT_TOKENS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/comment-tokens.json"
));

// Marker for start/end of header, blank unicode joiner.
const HEADER_MARKER: char = '\u{2060}';
const HEADER_MARKER_STR: &str = "\u{2060}"; // String version for searching

/// Renders a license template using Handlebars.
///
/// # Arguments
///
/// * `source`: The raw template string.
/// * `year`: The copyright year `Date`.
/// * `authors`: A list of author names.
///
/// # Returns
///
/// A `Result` containing the rendered string or a `RenderError`.
pub fn render_license(
    source: &str,
    year: &Date,
    authors: &Option<Authors>,
) -> Result<String, RenderError> {
    let mut handlebars = Handlebars::new();
    // Keep escape_fn default (HTML escaping) or consider no_escape if it's plain text only
    // handlebars.register_escape_fn(handlebars::no_escape); // Use if pure text output
    handlebars
        .register_template_string("license", source)
        .map_err(|e| {
            // Wrap the Box<TemplateError> into a RenderError for consistency
            RenderError::from(e)
        })?;

    let copyright_string;
    if let Some(authors) = authors {
        copyright_string = format!("Copyright (c) {} {}", year.year(), authors);
    } else {
        copyright_string = format!("Copyright (c) {}", year.year());
    }

    let mut data = BTreeMap::new();
    data.insert("copyright".to_string(), copyright_string);
    // Add other potential template variables here (e.g., full_date)
    // data.insert("year".to_string(), year.year().to_string());
    // data.insert("authors".to_string(), authors_list);

    handlebars.render("license", &data)
}

/// Recursively finds all files within the target paths, applying exclusions.
///
/// # Arguments
///
/// * `targets`: A list of starting files or directories.
/// * `exclude_regex`: An optional regex pattern to exclude files/directories.
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of valid file paths or a `FileProcessingError`.
pub fn get_valid_files(
    targets: &[PathBuf],
    exclude_regex: &Option<Regex>,
) -> Result<Vec<PathBuf>, LichenError> {
    debug!(
        "Searching for processable files starting from targets: {:?}. Exclude pattern: {:?}",
        targets, exclude_regex
    );
    let mut files_to_process = Vec::new();
    let mut seen_paths = HashSet::new(); // To handle potential overlaps or duplicates

    for target in targets {
        if let Some(re) = exclude_regex.as_ref() {
            if re.is_match(&target.to_string_lossy()) {
                debug!("Excluding target file {}", target.display());
                continue;
            }
        }
        if !target.exists() {
            error!("Target path does not exist: '{}'", target.display());
            return Err(LichenError::InvalidPath(
                target.to_string_lossy().to_string(),
            ));
        }

        debug!("Walking directory/file: '{}'", target.display());
        let walker = WalkDir::new(target).follow_links(true).into_iter(); // Follow symlinks

        // Apply the exclusion filter during the walk
        let filtered_walker = walker.filter_entry(|entry| {
            let path = entry.path();
            let path_string = &path.to_string_lossy();
            // Normalize paths.
            let path_string = path_string.replace(MAIN_SEPARATOR, "/");
            trace!("Considering entry: '{}'", path.display());
            match exclude_regex {
                Some(regex) => {
                    // Check if the path string matches the exclusion regex
                    if regex.is_match(&path_string) {
                        debug!("Excluding path '{}' due to regex match.", path.display());
                        false // Exclude this entry and its children if it's a directory
                    } else {
                        true // Keep this entry
                    }
                }
                None => true, // No regex, keep everything
            }
        });

        for entry_result in filtered_walker {
            match entry_result {
                Ok(entry) => {
                    let path = entry.into_path(); // Consumes entry
                    // Only add files, not directories themselves
                    if path.is_file() {
                        trace!("Entry is a file: '{}'", path.display());
                        // Add to list if not seen before
                        if seen_paths.insert(path.clone()) {
                            trace!("Adding unique file to list: '{}'", path.display());
                            files_to_process.push(path);
                        } else {
                            warn!(
                                "Duplicate file path encountered and ignored: '{}'. This might happen if targets overlap.",
                                path.display()
                            );
                        }
                    } else {
                        trace!(
                            "Entry is not a file (likely a directory), skipping: '{}'",
                            path.display()
                        );
                    }
                }
                Err(walk_err) => {
                    // Log the error but try to continue if possible, unless it's fatal
                    let path_display = walk_err
                        .path()
                        .map_or_else(|| "unknown path".to_string(), |p| p.display().to_string());
                    error!(
                        "Error accessing entry during directory walk at or near '{}': {}",
                        path_display, walk_err
                    );
                    // Optionally return the error immediately:
                    // return Err(FileProcessingError::WalkdirError(walk_err));
                }
            }
        } // End inner loop for walker results
    } // End outer loop for targets

    if files_to_process.is_empty() {
        warn!("No files found matching the criteria in the specified targets and exclusions.");
    } else {
        info!(
            "Found {} files to process across all targets.",
            files_to_process.len()
        );
        trace!("Files identified for processing: {:?}", files_to_process);
    }

    Ok(files_to_process)
}

/// Looks up the appropriate comment tokens for a given file extension.
/// Reads from a JSON configuration file (`comment-tokens.json`) in the data directory.
///
/// # Arguments
///
/// * `extension`: The file extension (e.g., "rs", "py", "js").
///
/// # Returns
///
/// A `Result` containing a `Vec<CommentToken>` or a `FileProcessingError`.
/// Returns an empty Vec and logs a warning if the extension is not found.
/// Returns an error if the JSON file is missing or malformed.
pub fn get_comment_tokens_for_ext(extension: &str) -> Result<Vec<CommentToken>, LichenError> {
    trace!(
        "Looking up comment character for extension: '{}' using embedded JSON",
        extension
    );
    let mut tokens = Vec::new();

    trace!("Parsing embedded JSON for comment tokens.");
    // Parse the embedded JSON string directly
    let data: serde_json::Value = match serde_json::from_str(COMMENT_TOKENS_JSON) {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to parse embedded comment-tokens.json: {}", e);
            // Return the JSON parsing error wrapped in LichenError
            return Err(LichenError::JsonError(e));
        }
    };

    let languages_map = data.as_object().ok_or_else(|| {
        LichenError::Msg("Invalid embedded JSON format: Top level is not an object.".to_string())
    })?;

    trace!(
        "Searching for extension '{}' in parsed embedded JSON data.",
        extension
    );

    // Iterate through all of the language definitions
    for (_language_name, language_details) in languages_map {
        // Check if this language definition has 'file_types'
        if let Some(file_types_val) = language_details.get("file_types") {
            // Check if 'file_types' is an array
            if let Some(file_types_array) = file_types_val.as_array() {
                // Check if the target extension is in this language's file_types array
                let has_extension = file_types_array
                    .iter()
                    .filter_map(|v| v.as_str()) // Only consider string values in the array
                    .any(|ext_str| ext_str == extension);

                if has_extension {
                    trace!(
                        "Found matching extension '{}' under language entry.",
                        extension
                    );
                    // |1| try to parse the single-line comment
                    if let Some(val) = language_details.get("comment_token") {
                        match val.as_str() {
                            Some(s) => {
                                debug!("Found comment_token='{}' for extension '{}'", s, extension);
                                tokens.push(CommentToken::Line(s.to_owned()));
                            }
                            None => warn!(
                                "'comment_token' for extension '{}' is not a string, skipping",
                                extension
                            ),
                        }
                    }

                    // |2| try to parse all block-comment tokens
                    if let Some(val) = language_details.get("block_comment_tokens") {
                        // object case
                        if let Some(obj) = val.as_object() {
                            let start = obj.get("start").and_then(|v| v.as_str());
                            let end = obj.get("end").and_then(|v| v.as_str());

                            match (start, end) {
                                (Some(s), Some(e)) => {
                                    debug!(
                                        "Block comments start with `{}` and end with `{}`",
                                        s, e
                                    );
                                    tokens.push(CommentToken::Block {
                                        start: s.to_owned(),
                                        end: e.to_owned(),
                                    });
                                }
                                _ => {
                                    warn!(
                                        "`block_comment_tokens` for extension '{}' is missing \
                     'start' or 'end' string.",
                                        extension
                                    );
                                }
                            }
                        }
                        // array case
                        else if let Some(arr) = val.as_array() {
                            for (idx, item) in arr.iter().enumerate() {
                                if let Some(obj) = item.as_object() {
                                    let start = obj.get("start").and_then(|v| v.as_str());
                                    let end = obj.get("end").and_then(|v| v.as_str());

                                    match (start, end) {
                                        (Some(s), Some(e)) => {
                                            debug!(
                                                "Block comment #{} starts with `{}` and ends with `{}`",
                                                idx, s, e
                                            );
                                            tokens.push(CommentToken::Block {
                                                start: s.to_owned(),
                                                end: e.to_owned(),
                                            });
                                        }
                                        _ => {
                                            warn!(
                                                "`block_comment_tokens[{}]` for extension '{}' is missing \
                             'start' or 'end'.",
                                                idx, extension
                                            );
                                        }
                                    }
                                } else {
                                    warn!(
                                        "`block_comment_tokens[{}]` for extension '{}' is not an object.",
                                        idx, extension
                                    );
                                }
                            }
                        }
                        // neither object nor array
                        else {
                            warn!(
                                "`block_comment_tokens` for extension '{}' is neither an object \
             nor an array.",
                                extension
                            );
                        }
                    }

                    // Found the extension, return the tokens (even if empty)
                    return Ok(tokens);
                }
            } else {
                warn!("'file_types' for language entry is not an array, skipping.");
            }
        }
    }

    warn!(
        "Extension '{}' not found in embedded comment tokens data. Cannot determine comment token.",
        extension
    );
    // If no matching language/extension was found after checking all entries
    // Return Ok with empty vec, indicating no tokens found for this extension
    Ok(tokens)
}

/// Formats the raw license header text by prepending the appropriate comment syntax.
///
/// # Arguments
///
/// * `header_content`: The raw license header text.
/// * `comment_tokens`: A list of available `CommentToken`s for the file type.
/// * `prefers_block`: Whether to prefer block comments if available.
/// * `separator`: The character to append after the header block (e.g., SOT marker).
///
/// # Returns
///
/// A `String` containing the formatted header, or `None` if no suitable comment token is found.
pub fn format_header_with_comments(
    header_content: &str,
    comment_tokens: &[CommentToken],
    prefers_block: bool,
    separator: char,
) -> Option<String> {
    trace!(
        "Determining comment token from options: '{:?}', prefers_block: {}",
        comment_tokens, prefers_block
    );

    // Attempt to find preferred variant
    let chosen_token = comment_tokens
        .iter()
        .find(|ct| match ct {
            CommentToken::Block { .. } => prefers_block,
            CommentToken::Line(_) => !prefers_block,
        })
        .or_else(|| {
            // Fallback if preferred not found
            comment_tokens.iter().find(|ct| match ct {
                CommentToken::Block { .. } => !prefers_block, // Find the other type
                CommentToken::Line(_) => prefers_block,       // Find the other type
            })
        });

    let comment_token = match chosen_token {
        Some(token) => token,
        None => {
            warn!("No suitable comment token found in the provided list.");
            return None; // Indicate failure to format
        }
    };

    trace!(
        "Formatting header with chosen comment token: '{:?}'",
        comment_token
    );

    let mut formatted_header = String::new();
    let newline: char = '\n';

    match comment_token {
        CommentToken::Line(comment_token) => {
            // Break the content into lines
            // Ensure consistent line endings and handle potential empty input
            let lines: Vec<&str> = header_content.trim_end().lines().collect();
            let line_count = lines.len();

            // Iterates over each line of the provided license, prepending the token.
            // If the line has no content (Some kind of spacer), no content needs to be added, and just the comment char is left for continuity
            for (i, line) in lines.iter().enumerate() {
                formatted_header.push_str(comment_token);
                if i == 0 {
                    // A separater to denote the first line of the comment
                    formatted_header.push(separator);
                }
                // Add a space and complete the line only if the line isn't empty, otherwise just the token
                if !line.is_empty() {
                    formatted_header.push(' ');
                    formatted_header.push_str(line);
                }
                // Add newline except for the very last line where we add the separator
                if i < line_count - 1 {
                    formatted_header.push(newline);
                }
            }
            // Append separator and final newline after the loop, marking the last line
            formatted_header.push(separator);
        }
        CommentToken::Block { start, end } => {
            formatted_header.push(newline); // Spacer newline
            formatted_header.push_str(start);
            formatted_header.push(separator); // The first line of the block
            formatted_header.push(newline); // Add newline after start token
            formatted_header.push_str(header_content.trim()); // Trim whitespace
            formatted_header.push(newline); // Add newline before end token
            formatted_header.push(separator); // The last line of the block
            formatted_header.push_str(end);
            formatted_header.push(newline); // Padding newline
        }
    }

    debug!("Header formatting complete.");
    Some(formatted_header)
}

pub trait ReplaceBetween {
    fn replace_between<'a>(&'a self, delim: char, replacement: &str) -> Cow<'a, str>;
}

use std::borrow::Cow;
impl ReplaceBetween for str {
    fn replace_between<'a>(&'a self, delim: char, replacement: &str) -> Cow<'a, str> {
        let mut first_sight: Option<usize> = None;
        let mut last_seen: Option<usize> = None;

        // Collect all of the lines, compressing into a vector, for the slicing mechanisms.
        // This pass finds all of the indicies
        let all_lines: Vec<&str> = self
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if line.contains(delim) {
                    // If a first line doesn't already exist, take the first.
                    first_sight = first_sight.or(Some(index));
                    // Always update the last index found, as the most recent is the oldest.
                    last_seen = Some(index);
                }
                line // Returning for collection
            })
            .collect();

        // This pass detects if there are any lines containing the delimeter were found
        if let (Some(first_idx), Some(last_idx)) = (first_sight, last_seen) {
            // Lines were found to replace, so initialize a new string with Cow
            let mut result_parts: Vec<Cow<'_, str>> = Vec::new();

            // |1| Add the lines *before* the first matched line index
            if first_idx > 0 {
                // Slice the collected lines from the beginning up to first_idx
                result_parts.push(Cow::Owned(all_lines[0..first_idx].join("\n")));
            }

            // |2| Add the replacement content
            result_parts.push(Cow::Borrowed(replacement));

            // |3| Add lines *after* the last matched line index
            if last_idx + 1 < all_lines.len() {
                // Slice the collected lines from one past the last line with delimeter until the very end
                result_parts.push(Cow::Owned(all_lines[last_idx + 1..].join("\n")));
            }

            // Join the parts with the newline.
            // Filter out any useless, empty, Cow strings resulting from lonely replacement
            let final_string = result_parts
                .iter()
                .filter(|s| !s.is_empty()) // Avoid joining empty parts
                .map(|s| s.as_ref()) // Convert Cow<str> to &str for joining
                .collect::<Vec<&str>>()
                .join("\n");

            let mut final_result = final_string;
            // Always end with a trailing newline
            final_result.push('\n');

            Cow::Owned(final_result)
        } else {
            // No lines contained the delimiter, return the original text slice
            // without allocating a new String.
            Cow::Borrowed(self)
        }
    }
}

/// Removes license headers previously marked by HEADER_MARKER from files asynchronously.
/// Modifies files directly. It identifies the header as the content between the start
/// of the file (or after a shebang) and the HEADER_MARKER.
///
/// # Arguments
///
/// * `paths`: A slice of `PathBuf` representing the files to modify.
/// * `max_concurrency`: The maximum number of files to process concurrently.
///
/// # Returns
///
/// A `Result<(), LichenError>` indicating overall success or the first error encountered.
pub async fn remove_headers_from_files(
    paths: &[PathBuf],
    max_concurrency: std::num::NonZero<usize>,
) -> Result<(), LichenError> {
    use futures::stream::{self, StreamExt}; // Ensure futures is imported
    use tokio::fs; // Use tokio's async fs

    debug!(
        "Starting to remove headers from {} files with concurrency {}",
        paths.len(),
        max_concurrency
    );

    let results = stream::iter(paths.to_owned())
        .map(|path| {
            async move {
                trace!("Processing file for header removal: '{}'", path.display());

                // |1| Skip directories
                if path.is_dir() {
                    warn!("Skipping directory during removal: '{}'", path.display());
                    // Return Ok with stats: (removed, skipped, errors)
                    return Ok((0, 1, 0));
                }

                // |2| Read file content as string.
                let content = match fs::read_to_string(&path).await {
                    Ok(c) => c,
                    Err(e) => {
                        warn!(
                            "Failed to read '{}' for removal: {}. Skipping.",
                            path.display(),
                            e
                        );
                        // Return Ok with stats
                        return Ok((0, 1, 0));
                    }
                };

                // |3| Handle Shebang
                let mut shebang_len = 0;
                if content.starts_with("#!") {
                    // Find the first newline character
                    if let Some(pos) = content.find('\n') {
                        // Length includes the newline
                        shebang_len = pos + 1;
                    } else {
                        // The whole file is just a shebang line (unlikely but possible)
                        // In this case, no header can exist after it.
                        trace!(
                            "File '{}' is only a shebang line. Skipping removal.",
                            path.display()
                        );
                        return Ok((0, 1, 0)); // Skip
                    }
                }

                // |4| Find the LAST header marker *after* the shebang (if any)
                let search_area = &content[shebang_len..];
                let marker_pos_in_search_area = search_area.rfind(HEADER_MARKER_STR);

                if let Some(relative_pos) = marker_pos_in_search_area {
                    // Calculate absolute position of the marker in the original content
                    let marker_start_pos = shebang_len + relative_pos;
                    // Calculate the position *after* the marker
                    let content_after_marker_pos = marker_start_pos + HEADER_MARKER_STR.len();

                    // Construct the new content: shebang (if any) + content after marker
                    let mut new_text = String::with_capacity(
                        shebang_len + (content.len() - content_after_marker_pos),
                    );

                    // Add shebang if it exists
                    if shebang_len > 0 {
                        new_text.push_str(&content[..shebang_len]);
                    }

                    // Add the rest of the content, trimming potential leading newline
                    // left over from the header removal.
                    let rest_content = &content[content_after_marker_pos..];
                    new_text.push_str(rest_content.trim_start_matches('\n'));

                    // |5| Write the modified content back to the file
                    match fs::write(&path, new_text).await {
                        Ok(_) => {
                            info!("Removed header from '{}'", path.display());
                            Ok((1, 0, 0)) // Removed
                        }
                        Err(e) => {
                            error!(
                                "Failed to write removed header to '{}': {}",
                                path.display(),
                                e
                            );
                            Ok((0, 0, 1)) // Error
                        }
                    }
                } else {
                    // Header marker not found after shebang (or at start if no shebang)
                    info!(
                        "Header marker not found in '{}'. Skipping removal.",
                        path.display()
                    );
                    Ok((0, 1, 0)) // Skip
                }
            }
        })
        .buffer_unordered(max_concurrency.into()) // Process concurrently
        .collect::<Vec<Result<(usize, usize, usize), LichenError>>>() // Collect results
        .await;

    // Aggregate results and check for errors
    let mut total_removed = 0;
    let mut total_skipped = 0;
    let mut total_errors = 0;
    let mut first_error: Option<LichenError> = None;

    for result in results {
        match result {
            Ok((removed, skipped, errors)) => {
                total_removed += removed;
                total_skipped += skipped;
                total_errors += errors;
            }
            Err(e) => {
                // This handles errors from the async block itself returning Err
                error!("Unexpected error during stream processing: {}", e);
                total_errors += 1;
                if first_error.is_none() {
                    first_error = Some(e);
                }
            }
        }
    }

    info!(
        "Header removal summary: {} removed, {} skipped, {} errors.",
        total_removed, total_skipped, total_errors
    );

    if total_errors > 0 {
        // Return the first specific error encountered, or a generic one
        Err(first_error.unwrap_or_else(|| {
            LichenError::Msg(format!(
                "Encountered {} errors during header removal.",
                total_errors
            ))
        }))
    } else {
        Ok(())
    }
}

/// Applies the license header to a list of files asynchronously.
/// Modifies files directly.
///
/// # Arguments
///
/// * `header_content`: The license header text (raw, without comment markers).
/// * `paths`: A slice of `PathBuf` representing the files to modify.
/// * `max_concurrency`: The maximum number of files to process concurrently.
/// * `prefers_block`: Whether to prefer block comments.
///
/// # Returns
///
/// A `Result<(), FileProcessingError>` indicating overall success or the first error encountered.
pub async fn apply_headers_to_files(
    header_content: &str,
    paths: &[PathBuf],
    max_concurrency: std::num::NonZero<usize>,
    prefers_block: bool,
    multiple: bool,
) -> Result<(), LichenError> {
    use futures::stream::{self, StreamExt};
    use tokio::fs; // Ensure futures is imported
    debug!(
        "Starting to apply headers to {} files with concurrency {}",
        paths.len(),
        max_concurrency
    );

    // Share header content safely across tasks
    let header_content_arc = Arc::new(header_content.to_string());

    let results = stream::iter(paths.to_owned())
        .map(|path| {
            let header_content = header_content_arc.clone(); // Clone Arc, not the String
            async move {
                trace!("Processing file: '{}'", path.display());

                // |1| Directories cannot be written to skip.
                if path.is_dir() {
                    warn!("Skipping directory: '{}'", path.display());
                    // Return Ok with stats: (applied, skipped, errors)
                    return Ok((0, 1, 0));
                }

                // |2| Read file content as string.
                let content = match fs::read_to_string(&path).await {
                    Ok(c) => c,
                    Err(e) => {
                        warn!("Failed to read '{}': {}. Skipping.", path.display(), e);
                        // Return Ok with stats
                        return Ok((0, 1, 0));
                    }
                };

                // |4| Find comment token for extension
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let comment_tokens = match get_comment_tokens_for_ext(ext) {
                    Ok(tokens) if !tokens.is_empty() => tokens,
                    Ok(_) => {
                        // No tokens found for this extension
                        warn!(
                            "No comment tokens defined for extension '{}' (file '{}'). Skipping.",
                            ext,
                            path.display()
                        );
                        return Ok((0, 1, 0)); // Skip
                    }
                    Err(e) => {
                        // Error occurred trying to get tokens (e.g., JSON read error)
                        error!(
                            "Failed to get comment tokens for '{}': {}. Skipping.",
                            path.display(),
                            e
                        );
                        return Ok((0, 0, 1)); // Error
                    }
                };

                // |5| Header formatting
                let formatted_header = match format_header_with_comments(
                    &header_content,
                    &comment_tokens,
                    prefers_block,
                    HEADER_MARKER,
                ) {
                    Some(h) => h,
                    None => {
                        // This case should be rare if get_comment_tokens_for_ext returned non-empty
                        error!(
                            "Failed to format header for '{}' (no suitable token found). Skipping.",
                            path.display()
                        );
                        return Ok((0, 1, 0)); // Skip
                    }
                };

                // If multiple, do not overwrite any headers
                if !multiple {
                    // |6| If header is already present, simply replace it.
                    if content.contains(HEADER_MARKER) {
                        info!(
                            "Already contains header marker, replacing '{}'",
                            path.display()
                        );
                        let content = content.replace_between(HEADER_MARKER, &formatted_header);
                        fs::write(&path, content.to_string()).await?;
                        // Return Ok with stats
                        return Ok((1, 0, 0));
                    }
                }

                // |6| Shebang handling
                let (shebang, rest) = if content.starts_with("#!") {
                    // Find the first newline character
                    if let Some(pos) = content.find('\n') {
                        // Split at the position *after* the newline
                        let (sb, rem) = content.split_at(pos + 1);
                        (Some(sb), rem)
                    } else {
                        // The whole file is just a shebang line (unlikely but possible)
                        (Some(content.as_str()), "")
                    }
                } else {
                    (None, content.as_str())
                };

                // |7| Create buffer for text and write to it
                let mut new_text = String::with_capacity(
                    shebang.map_or(0, |s| s.len()) +
                    formatted_header.len() +
                    1 + // Potential extra newline
                    rest.len(),
                );

                if let Some(sb) = shebang {
                    new_text.push_str(sb);
                    // Ensure the rest starts on a new line if shebang didn't end with one
                    if !sb.ends_with('\n') {
                        new_text.push('\n');
                    }
                }
                new_text.push_str(&formatted_header);
                // Ensure there's a newline between header and original content
                if !formatted_header.ends_with('\n') {
                    new_text.push('\n');
                }
                // Avoid double newline if original content already starts with one
                new_text.push_str(rest.trim_start_matches('\n'));

                // |8| Write the modified content back to the file
                match fs::write(&path, new_text).await {
                    Ok(_) => {
                        info!("Applied header to '{}'", path.display());
                        Ok((1, 0, 0)) // Applied
                    }
                    Err(e) => {
                        error!("Failed to write header to '{}': {}", path.display(), e);
                        Ok((0, 0, 1)) // Error
                    }
                }
            }
        })
        .buffer_unordered(max_concurrency.into()) // Process concurrently
        .collect::<Vec<Result<(usize, usize, usize), LichenError>>>() // Collect results
        .await;

    // Aggregate results and check for errors
    let mut total_applied = 0;
    let mut total_skipped = 0;
    let mut total_errors = 0;
    let mut first_error: Option<LichenError> = None;

    for result in results {
        match result {
            Ok((applied, skipped, errors)) => {
                total_applied += applied;
                total_skipped += skipped;
                total_errors += errors;
            }
            Err(e) => {
                // This happens if the async block itself returns Err,
                // which we avoided by returning Ok((a,s,e))
                // But keep it here for robustness.
                error!("Unexpected error during stream processing: {}", e);
                total_errors += 1;
                if first_error.is_none() {
                    first_error = Some(e);
                }
            }
        }
    }

    info!(
        "Header application summary: {} applied, {} skipped, {} errors.",
        total_applied, total_skipped, total_errors
    );

    if total_errors > 0 {
        // Return the first specific error encountered, or a generic one
        Err(first_error.unwrap_or_else(|| {
            LichenError::Msg(format!(
                "Encountered {} errors during header application.",
                total_errors
            ))
        }))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Author;
    use crate::config::Authors;
    use jiff::civil::Date;
    use std::borrow::Cow;

    #[test]
    fn render_license_injects_copyright_and_authors() {
        let template = "/* {{copyright}} */";
        let authors = Some(Authors(vec![
            Author {
                name: "A".into(),
                email: None,
            },
            Author {
                name: "B".into(),
                email: Some("b@e".into()),
            },
        ]));
        let year = Date::new(2025, 1, 1).unwrap();
        let out = render_license(template, &year, &authors).unwrap();
        assert!(out.contains("2025"));
        assert!(out.contains("A"));
        assert!(out.contains("B [b@e]"));
    }

    #[test]
    fn replace_between_replaces_delimited_region() {
        let text = "line1\n* old\n* old2\nline4";
        let replaced = text.replace_between('*', "NEW\nNEW2");
        // expect lines before first '*', then our replacement, then lines after last '*'
        let expect = "line1\nNEW\nNEW2\nline4\n";
        match replaced {
            Cow::Owned(ref s) => assert_eq!(s, expect),
            _ => panic!("expected owned String"),
        }
    }

    #[test]
    fn replace_between_no_delimiter_returns_borrowed() {
        let text = "no markers here";
        let replaced = text.replace_between('#', "X");
        // without marker, should return the original slice
        assert!(matches!(replaced, Cow::Borrowed(_)));
        assert_eq!(replaced, "no markers here");
    }
}

fn load_gitignore_patterns() -> Result<Option<Vec<String>>, LichenError> {
    let mut patterns = Vec::new();
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        // Deal with errors
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git command failed: {}", stderr).into());
    }

    let project_directory = String::from_utf8(output.stdout).unwrap().trim().to_string();

    // Build a PathBuf to the `.gitignore`
    let gitignore = PathBuf::from(project_directory).join(".gitignore");

    // If there's no .gitignore, return none
    let content = match fs::read_to_string(gitignore) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };

    for line in content.lines() {
        let line = line.trim();
        // skip comments and blank lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // detect directory‐only patterns ending with '/'
        let is_dir = line.ends_with('/');
        let pat = if is_dir {
            // strip trailing '/'
            &line[..line.len() - 1]
        } else {
            line
        };

        // escape regex metachars, then re‐inject glob semantics
        let mut re = regex::escape(pat).replace(r"\*", ".*").replace(r"\?", ".");

        // if it was a directory pattern, match any descendant
        if is_dir {
            re.push_str("/.*");
        }

        patterns.push(re);
    }

    Ok(Some(patterns))
}

pub fn build_exclude_regex(
    cli_exclude: &Option<Regex>,
    cfg: Option<&Config>,
    all: bool,
    index: Option<usize>,
) -> Result<Option<Regex>, LichenError> {
    // 1) collect all raw pattern strings
    let mut pats = Vec::new();

    let defaults: Vec<String> = vec![
        "\\.gitignore".to_string(),
        ".*lock".to_string(),
        "\\.git/.*".to_string(),
        "\\.licensure\\.yml".to_string(),
        "README.*".to_string(),
        "LICENSE.*".to_string(),
        ".*\\.(md|rst|txt)".to_string(),
        "Cargo.toml".to_string(),
        ".*\\.github/.*".to_string(),
    ];

    // a) add .gitignore patterns (unless disabled)
    if !all {
        if let Some(gitignore) = load_gitignore_patterns()? {
            pats.extend(gitignore);
        }

        pats.extend(defaults);
    }

    if let Some(cfg) = cfg {
        // b) global exclude from config
        if let Some(globs) = cfg.exclude.as_ref() {
            for re in globs.iter() {
                pats.push(re.as_str().to_string());
            }
        }

        // c) per‑license exclude
        if let Some(i) = index {
            if let Some(licenses) = cfg.licenses.as_ref() {
                if let Some(lic) = licenses.get(i) {
                    // If the license exists, check if it has an exclude pattern
                    if let Some(exc) = lic.exclude.as_ref() {
                        pats.push(exc.to_string());
                    }
                } else {
                    // If the index is provided but out of bounds for the licenses vec,
                    return Err(LichenError::InvalidIndex(i));
                }
            }
            // If cfg.licenses was None, do nothing.
        }
        // If no cfg, do nothing
    }

    // d) CLI override
    if let Some(cli_exc) = cli_exclude {
        pats.push(cli_exc.to_string());
    }

    // nothing to exclude?
    if pats.is_empty() {
        return Ok(None);
    }

    // 2) wrap each in a non‑capturing group and join with |
    let alternation = pats
        .into_iter()
        .map(|p| format!("(?:{})", p))
        .collect::<Vec<_>>()
        .join("|");

    // 3) compile once
    match Regex::new(&alternation) {
        Ok(re) => Ok(Some(re)),
        Err(err) => Err(LichenError::RegexError(alternation, err)),
    }
}
