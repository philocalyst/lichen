//! # Utility Functions
//!
//! General helper functions for file processing, text formatting, etc.

use crate::config::{Author, Authors};
use crate::error::LichenError;
use crate::models::CommentToken;
use handlebars::{Handlebars, RenderError};
use jiff::civil::Date;
use log::{debug, error, info, trace, warn};
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::PathBuf;
use walkdir::{self, WalkDir};

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
            trace!("Considering entry: '{}'", path.display());
            match exclude_regex {
                Some(regex) => {
                    // Check if the path string matches the exclusion regex
                    if regex.is_match(&path.to_string_lossy()) {
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
        "Looking up comment character for extension: '{}'",
        extension
    );
    let comment_tokens_path = crate::paths::get_comment_tokens_path()?;
    let mut tokens = Vec::new();

    // --- Ensure Data Directory and File Exist ---
    if let Some(parent_dir) = comment_tokens_path.parent() {
        if !parent_dir.exists() {
            debug!(
                "Data directory '{}' does not exist. Attempting to create.",
                parent_dir.display()
            );
            fs::create_dir_all(parent_dir)?; // Create parent dirs if needed
        }
    }
    if !comment_tokens_path.exists() {
        warn!(
            "Comment tokens file '{}' not found. Creating empty file. Cannot determine comment tokens.",
            comment_tokens_path.display()
        );
        fs::write(&comment_tokens_path, "{}")?; // Create an empty JSON object
        // Return Ok with empty vec, as the file *now* exists but has no data
        return Ok(tokens);
    }
    // ---

    trace!(
        "Opening comment tokens file: '{}'",
        comment_tokens_path.display()
    );
    let file = File::open(&comment_tokens_path)?;
    let reader = BufReader::new(file);

    trace!("Parsing JSON from '{}'", comment_tokens_path.display());
    let data: serde_json::Value = serde_json::from_reader(reader)?; // Propagate JSON parsing errors

    let languages_map = data.as_object().ok_or_else(|| {
        LichenError::Msg(format!(
            "Invalid JSON format in '{}': Top level is not an object.",
            comment_tokens_path.display()
        ))
    })?;

    trace!(
        "Searching for extension '{}' in parsed JSON data.",
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
                        // make sure it's an object
                        if let Some(obj) = val.as_object() {
                            // Use Option chaining and map_or_else for cleaner error handling
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
                                        "`block_comment_tokens` for extension '{}' is missing 'start' or 'end' string.",
                                        extension
                                    );
                                }
                            }
                        } else {
                            warn!(
                                "`block_comment_tokens` for extension '{}' is not an object.",
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

    // If no matching language/extension was found after checking all entries
    warn!(
        "Extension '{}' not found in comment tokens file '{}'. Cannot determine comment token.",
        extension,
        comment_tokens_path.display()
    );
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

    match comment_token {
        CommentToken::Line(tok) => {
            // Ensure consistent line endings and handle potential empty input
            let lines: Vec<&str> = header_content.trim_end().lines().collect();
            let line_count = lines.len();

            for (i, line) in lines.iter().enumerate() {
                formatted_header.push_str(tok);
                // Add a space only if the line isn't empty, otherwise just the token
                if !line.is_empty() {
                    formatted_header.push(' ');
                    formatted_header.push_str(line);
                }
                // Add newline except for the very last line where we add the separator
                if i < line_count - 1 {
                    formatted_header.push('\n');
                }
            }
            // Append separator and final newline after the loop
            formatted_header.push(separator);
            formatted_header.push('\n');
        }
        CommentToken::Block { start, end } => {
            formatted_header.push_str(start);
            formatted_header.push('\n'); // Add newline after start token
            formatted_header.push_str(header_content.trim()); // Trim whitespace
            formatted_header.push('\n'); // Add newline before end token
            formatted_header.push_str(end);
            formatted_header.push(separator); // Mark
            formatted_header.push('\n'); // Padding newline
        }
    }

    debug!("Header formatting complete.");
    Some(formatted_header)
}

/// Applies the license header to a list of files asynchronously.
/// Modifies files directly (in-place).
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
    max_concurrency: usize,
    prefers_block: bool,
) -> Result<(), LichenError> {
    use futures::stream::{self, StreamExt}; // Ensure futures is imported
    use std::sync::Arc;
    use tokio::fs; // Use tokio's async fs

    debug!(
        "Starting to apply headers to {} files with concurrency {}",
        paths.len(),
        max_concurrency
    );

    // Marker for end of header (SOT - Start of Text, used somewhat unconventionally here)
    const HEADER_MARKER: char = '\x02';

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
                        error!("Failed to read '{}': {}. Skipping.", path.display(), e);
                        // Return Ok with stats
                        return Ok((0, 0, 1));
                    }
                };

                // |3| Skip if header marker already present
                if content.contains(HEADER_MARKER) {
                    info!(
                        "Already contains header marker, skipping '{}'",
                        path.display()
                    );
                    // Return Ok with stats
                    return Ok((0, 1, 0));
                }

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
        .buffer_unordered(max_concurrency) // Process concurrently
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
