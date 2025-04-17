//! # Lichen
//!
//! A license management cli tool.

// Std library imports
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// External crate imports
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use directories::ProjectDirs;
use log::{debug, error, info, trace, warn}; // Import specific log levels
use regex::Regex;
use serde_json;
use tempfile::NamedTempFile;
use walkdir::{self, WalkDir};

// Local module imports
mod license;
use license::License;

// --- Custom Error Types ---

/// Errors that can occur during file processing operations.
#[derive(Debug)]
pub enum FileProcessingError {
    /// An I/O error occurred.
    IoError(std::io::Error),
    /// An error occurred while walking a directory.
    WalkdirError(walkdir::Error),
    /// An invalid path was encountered.
    InvalidPath(String),
    /// Failed to parse JSON data.
    JsonError(serde_json::Error),
    /// Could not determine project directories.
    ProjectDirsError(String),
    /// Generic error message.
    Msg(String), // Added for more flexibility
}

// --- Error Trait Implementations ---

impl fmt::Display for FileProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileProcessingError::IoError(err) => write!(f, "IO error: {}", err),
            FileProcessingError::WalkdirError(err) => write!(f, "Directory walk error: {}", err),
            FileProcessingError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            FileProcessingError::JsonError(err) => write!(f, "JSON processing error: {}", err),
            FileProcessingError::ProjectDirsError(err) => {
                write!(f, "Project directory error: {}", err)
            }
            FileProcessingError::Msg(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for FileProcessingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileProcessingError::IoError(err) => Some(err),
            FileProcessingError::WalkdirError(err) => Some(err),
            FileProcessingError::JsonError(err) => Some(err),
            _ => None, // Other variants don't wrap another error directly
        }
    }
}

// --- Error Conversion Implementations ---

impl From<std::io::Error> for FileProcessingError {
    fn from(err: std::io::Error) -> Self {
        FileProcessingError::IoError(err)
    }
}

impl From<walkdir::Error> for FileProcessingError {
    fn from(err: walkdir::Error) -> Self {
        FileProcessingError::WalkdirError(err)
    }
}

impl From<serde_json::Error> for FileProcessingError {
    fn from(err: serde_json::Error) -> Self {
        FileProcessingError::JsonError(err)
    }
}

impl From<&str> for FileProcessingError {
    fn from(msg: &str) -> Self {
        FileProcessingError::Msg(msg.to_string())
    }
}

impl From<String> for FileProcessingError {
    fn from(msg: String) -> Self {
        FileProcessingError::Msg(msg)
    }
}

// --- CLI Argument Structs ---

/// A license management cli tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a license file (e.g., LICENSE.txt or LICENSE.md)
    Gen(GenArgs),
    /// Apply license headers to source files
    Apply(ApplyArgs),
    /// Initialize a default configuration file (Not fully implemented)
    Init(InitArgs),
}

#[derive(Args, Debug)]
struct GenArgs {
    /// SPDX identifier of the license to generate (e.g., MIT, Apache-2.0).
    /// Can be omitted if specified in configuration.
    #[arg()]
    license: Option<License>,

    /// Author names (comma-separated).
    #[arg(short, long, value_delimiter = ',')]
    author: Option<Vec<String>>,

    /// Generate a Markdown formatted license file (`LICENSE.md`). Defaults to plain text (`LICENSE.txt`).
    #[arg(long, default_value_t = false)] // Default to false for .txt
    markdown: bool,

    /// Year for the license copyright notice (defaults to the current year).
    #[arg(short, long)]
    year: Option<u16>,
}

#[derive(Args, Debug)]
struct ApplyArgs {
    /// SPDX identifier of the license header to apply (e.g., MIT, Apache-2.0).
    /// Can be omitted if specified in configuration.
    #[arg()]
    license: Option<License>,

    /// Apply headers in-place, modifying the original files.
    /// Caution: This modifies files directly. Ensure backups or version control.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    in_place: bool,

    /// Regex pattern for files/directories to exclude. Applied during directory traversal.
    #[arg(short, long)] // Removed value_delimiter, regex parsing handles it
    exclude: Option<Regex>,

    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1.., default_value = ".")]
    target: Vec<PathBuf>,
}

#[derive(Args, Debug)]
struct InitArgs {
    /// Optional path where the configuration should be initialized.
    /// Defaults to the current directory.
    #[arg(short, long)]
    path: Option<PathBuf>,
}

// --- Main Application Logic ---

fn main() -> ExitCode {
    // 1. Parse CLI arguments
    let cli = Cli::parse();

    // 2. Initialize logging
    // Uses clap_verbosity_flag to set level based on -v, -vv, etc.
    // Also respects RUST_LOG environment variable.
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Lichen starting...");
    debug!("CLI arguments parsed: {:?}", cli);

    // --- Configuration Loading Placeholder ---
    // TODO: Implement robust configuration loading (e.g., from .lichenrc, XDG dirs)
    // TODO: Merge CLI arguments with configuration (CLI takes precedence).
    debug!("Configuration loading step (currently placeholder).");
    // ---

    // 3. Dispatch to appropriate subcommand handler
    let result = match cli.command {
        Commands::Gen(args) => {
            info!("Executing 'gen' command.");
            run_gen(args)
        }
        Commands::Apply(args) => {
            info!("Executing 'apply' command.");
            run_apply(args)
        }
        Commands::Init(args) => {
            info!("Executing 'init' command.");
            run_init(args)
        }
    };

    // 4. Handle command result and exit
    match result {
        Ok(_) => {
            info!("Command executed successfully.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            error!("Command failed: {}", e);
            // Log the error source if available for deeper debugging
            if let Some(source) = e.source() {
                error!("Caused by: {}", source);
            }
            ExitCode::FAILURE
        }
    }
}

// --- Helper Functions ---

/// Gets the application's data directory using the `directories` crate.
///
/// This typically corresponds to standard locations like:
/// - Linux: `$HOME/.local/share/lichen`
/// - macOS: `$HOME/Library/Application Support/com.philocalyst.lichen`
/// - Windows: `%APPDATA%\philocalyst\lichen\data`
///
/// # Returns
///
/// Returns a `Result` containing the `PathBuf` to the data directory or a `FileProcessingError`.
fn get_data_dir() -> Result<PathBuf, FileProcessingError> {
    trace!("Attempting to determine project directories.");
    let proj_dirs = ProjectDirs::from("com", "philocalyst", "lichen").ok_or_else(|| {
        FileProcessingError::ProjectDirsError(
            "Could not determine application directories.".to_string(),
        )
    })?;

    let data_dir = proj_dirs.data_dir();
    debug!("Determined data directory: '{}'", data_dir.display());
    Ok(data_dir.to_path_buf())
}

/// Constructs the full path to a specific license template file.
///
/// # Arguments
///
/// * `license`: The `License` enum variant.
/// * `extension`: The desired file extension (e.g., "txt", "md", "header.txt").
///
/// # Returns
///
/// Returns a `Result` containing the `PathBuf` to the license file or a `FileProcessingError`.
fn get_license_path(license: &License, extension: &str) -> Result<PathBuf, FileProcessingError> {
    trace!(
        "Constructing license path for license '{}' with extension '{}'",
        license.spdx_id(),
        extension
    );
    let data_dir = get_data_dir()?;
    // Expected structure: <data_dir>/licenses/<spdx_id>.<extension>
    let path = data_dir
        .join("licenses")
        .join(format!("{}.{}", license.spdx_id(), extension));
    debug!("Constructed license template path: '{}'", path.display());
    Ok(path)
}

/// Constructs the full path to the comment tokens JSON file.
///
/// # Returns
///
/// Returns a `Result` containing the `PathBuf` to the JSON file or a `FileProcessingError`.
fn get_comment_tokens_path() -> Result<PathBuf, FileProcessingError> {
    trace!("Constructing path for comment-tokens.json");
    let data_dir = get_data_dir()?;
    // Expected structure: <data_dir>/comment-tokens.json
    let path = data_dir.join("comment-tokens.json");
    debug!("Constructed comment tokens path: '{}'", path.display());
    Ok(path)
}

// --- Subcommand Implementations ---

/// Handles the `gen` command logic.
fn run_gen(args: GenArgs) -> Result<(), FileProcessingError> {
    debug!("Starting run_gen with args: {:?}", args);

    // --- Parameter Resolution (CLI vs. Config - Placeholder) ---
    // TODO: Load license, author, year from config if not provided in args.
    let license = args
        .license
        .ok_or("License SPDX ID is required via CLI or config for 'gen' command")?;
    let authors = args.author; // .or_else(|| /* get from config */);
    let year = args.year.unwrap_or_else(|| {
        let current_year = chrono::Utc::now()
            .format("%Y")
            .to_string()
            .parse()
            .unwrap_or(2024); // Provide a fallback year
        debug!(
            "Year not specified, defaulting to current year: {}",
            current_year
        );
        current_year
    });
    let extension = if args.markdown { "md" } else { "txt" };
    // ---

    info!(
        "Generating license file for: {}, Year: {}, Format: {}",
        license.spdx_id(),
        year,
        extension
    );
    if let Some(ref authors_vec) = authors {
        info!("Authors specified: {}", authors_vec.join(", "));
    } else {
        debug!("No authors specified via CLI.");
        // TODO: Log authors from config if loaded
    }

    // --- Template Fetching ---
    // TODO: Implement actual template processing (placeholder substitution)
    //       Currently, it just copies the raw template.
    let license_template_path = get_license_path(&license, extension)?;
    if !license_template_path.exists() {
        error!(
            "License template file not found at '{}'. Ensure templates are installed correctly.",
            license_template_path.display()
        );
        return Err(FileProcessingError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "License template not found: {}",
                license_template_path.display()
            ),
        )));
    }
    debug!(
        "Found license template file at: '{}'",
        license_template_path.display()
    );
    // ---

    // --- Output File Generation ---
    let mut output_filename = PathBuf::from("LICENSE");
    // For txt files the standard is no extention
    if extension == "md" {
        output_filename.set_extension("md"); // Without the file extention most programs just read as text...
    }
    debug!(
        "Determined output filename: '{}'",
        output_filename.display()
    );

    // TODO: Use 'processed_content' once substitution is implemented
    // fs::write(&output_filename, processed_content)?;

    // Current behavior: Copy the raw template
    match fs::copy(&license_template_path, &output_filename) {
        Ok(bytes_copied) => {
            info!(
                "Successfully generated license file '{}' ({} bytes copied from template).",
                output_filename.display(),
                bytes_copied
            );
        }
        Err(e) => {
            error!(
                "Failed to copy license template from '{}' to '{}': {}",
                license_template_path.display(),
                output_filename.display(),
                e
            );
            return Err(e.into());
        }
    }
    // ---

    Ok(())
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
) -> Result<Vec<PathBuf>, FileProcessingError> {
    debug!(
        "Searching for processable files starting from targets: {:?}. Exclude pattern: {:?}",
        targets, exclude_regex
    );
    let mut files_to_process = Vec::new();
    let mut seen_paths = HashSet::new(); // To handle potential overlaps or duplicates

    for target in targets {
        if !target.exists() {
            error!("Target path does not exist: '{}'", target.display());
            return Err(FileProcessingError::InvalidPath(
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
                    // Optionally, return the error immediately if desired:
                    // return Err(FileProcessingError::WalkdirError(walk_err));
                    // Current behavior: Log and continue (best effort)
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

/// Handles the `apply` command logic.
fn run_apply(args: ApplyArgs) -> Result<(), FileProcessingError> {
    debug!("Starting run_apply with args: {:?}", args);

    // --- Parameter Resolution (CLI vs. Config - Placeholder) ---
    // TODO: Load license, exclude_pattern, etc. from config if not in args.
    let license = args
        .license
        .ok_or("License SPDX ID is required via CLI or config for 'apply' command")?;
    let exclude_pattern = &args.exclude;
    let targets = &args.target;
    let in_place = args.in_place;
    // ---

    info!(
        "Applying license header for: {} to targets: {:?}",
        license.spdx_id(),
        targets
    );
    info!("In-place modification: {}", in_place);
    if in_place {
        warn!("Running with --in-place flag. Files will be modified directly.");
    }
    info!("Exclusion pattern: {:?}", exclude_pattern);

    // --- Get License Header Content ---
    // Headers usually use a plain text variant.
    let header_template_path = get_license_path(&license, "txt")?;
    if !header_template_path.exists() {
        error!(
            "License header template file not found at '{}'. Expected naming like 'SPDXID.header.txt'.",
            header_template_path.display()
        );
        return Err(FileProcessingError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "License header template not found: {}",
                header_template_path.display()
            ),
        )));
    }
    debug!(
        "Reading license header content from: '{}'",
        header_template_path.display()
    );
    let license_header_content = match fs::read_to_string(&header_template_path) {
        Ok(content) => content,
        Err(e) => {
            error!(
                "Failed to read license header template '{}': {}",
                header_template_path.display(),
                e
            );
            return Err(e.into());
        }
    };
    trace!(
        "License header content read successfully:\n{}",
        license_header_content
    ); // Use trace for potentially large content
    // ---

    // --- Find Files ---
    let files_to_process = get_valid_files(targets, exclude_pattern)?;
    if files_to_process.is_empty() {
        info!("No files require processing. Exiting 'apply' command.");
        return Ok(()); // Nothing to do
    }
    // ---

    // --- Apply Headers ---
    if in_place {
        info!(
            "Applying license headers in-place to {} files...",
            files_to_process.len()
        );
        apply_headers_to_files(&license_header_content, &files_to_process)?;
    } else {
        // TODO: Implement copying files to a 'licensed/' directory and applying headers there.
        error!(
            "Non-in-place application (copying to 'licensed/' directory) is not yet implemented."
        );
        println!(
            "Placeholder: Would apply license '{}' header to {} files (creating copies).",
            license.spdx_id(),
            files_to_process.len()
        );
        // For now, return an error or just skip
        return Err(FileProcessingError::Msg(
            "Non-in-place application not implemented.".to_string(),
        ));
    }
    // ---

    info!("Finished applying license headers.");
    Ok(())
}

/// Applies the license header to a list of files.
/// Modifies files directly (in-place).
///
/// # Arguments
///
/// * `header_content`: The license header text (raw, without comment markers).
/// * `paths`: A slice of `PathBuf` representing the files to modify.
///
/// # Returns
///
/// An `io::Result<()>` indicating success or failure.
fn apply_headers_to_files(
    header_content: &str,
    paths: &[PathBuf],
) -> Result<(), FileProcessingError> {
    debug!("Starting to apply headers to {} files.", paths.len());
    let mut applied_count = 0;
    let mut skipped_count = 0;
    let mut error_count = 0;

    // SOT character (ASCII 2) used as a marker for an existing header.
    const HEADER_MARKER: char = 2 as char;

    for path in paths {
        debug!("Processing file: '{}'", path.display());

        // Basic check: Skip directories (shouldn't be in the list from get_valid_files, but defensive)
        if path.is_dir() {
            warn!(
                "Skipping directory found in processing list: '{}'",
                path.display()
            );
            skipped_count += 1;
            continue;
        }

        // --- Check for Existing Header Marker ---
        // Read just enough to check for the marker efficiently if possible,
        // but reading the whole thing is simpler for now.
        let file_content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Failed to read file '{}' to check for existing header: {}",
                    path.display(),
                    e
                );
                error_count += 1;
                continue; // Skip this file
            }
        };

        if file_content.contains(HEADER_MARKER) {
            info!(
                "File '{}' already contains a header marker (SOT char). Skipping.",
                path.display()
            );
            skipped_count += 1;
            continue;
        }
        trace!(
            "No existing header marker found in '{}'. Proceeding with applying header.",
            path.display()
        );
        // ---

        // --- Determine Comment Style ---
        let extension = path
            .extension()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("");
        trace!(
            "Determining comment character for extension: '{}'",
            extension
        );
        let comment_char = match get_comment_char(extension) {
            Ok(token) => {
                debug!(
                    "Using comment token '{}' for file '{}'",
                    token,
                    path.display()
                );
                token
            }
            Err(e) => {
                error!(
                    "Could not determine comment token for '{}' (extension '{}'): {}. Skipping file.",
                    path.display(),
                    extension,
                    e
                );
                error_count += 1;
                continue; // Skip this file
            }
        };
        // ---

        // --- Format Header ---
        trace!(
            "Formatting license header with comment token '{}'.",
            comment_char
        );
        let formatted_header =
            format_header_with_comments(header_content, &comment_char, HEADER_MARKER);
        // ---

        // --- Prepend Header to File ---
        trace!("Attempting to prepend header to file '{}'", path.display());
        match prepend_header_to_file(&formatted_header, path) {
            Ok(_) => {
                info!("Successfully applied header to '{}'", path.display());
                applied_count += 1;
            }
            Err(e) => {
                error!(
                    "Failed to prepend header to file '{}': {}",
                    path.display(),
                    e
                );
                error_count += 1;
                // Continue to the next file
            }
        }
        // ---
    }

    info!(
        "Header application summary: {} applied, {} skipped (already licensed), {} errors.",
        applied_count, skipped_count, error_count
    );

    if error_count > 0 {
        Err(FileProcessingError::Msg(format!(
            "Encountered {} errors during header application.",
            error_count
        )))
    } else {
        Ok(())
    }
}

/// Looks up the appropriate line comment character for a given file extension.
/// Reads from a JSON configuration file (`comment-tokens.json`) in the data directory.
///
/// # Arguments
///
/// * `extension`: The file extension (e.g., "rs", "py", "js").
///
/// # Returns
///
/// A `Result` containing the comment token `String` or a `FileProcessingError`.
/// Defaults to "#" if the extension is not found or the JSON is malformed/missing.
fn get_comment_char(extension: &str) -> Result<String, FileProcessingError> {
    trace!(
        "Looking up comment character for extension: '{}'",
        extension
    );
    let comment_tokens_path = get_comment_tokens_path()?;

    // --- Ensure Data Directory and File Exist ---
    // This check might be slightly redundant if get_data_dir succeeded, but ensures the json exists.
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
            "Comment tokens file '{}' not found. Creating empty file. Commenting may default to '#'.",
            comment_tokens_path.display()
        );
        fs::write(&comment_tokens_path, "{}")?; // Create an empty JSON object
    }
    // ---

    trace!(
        "Opening comment tokens file: '{}'",
        comment_tokens_path.display()
    );
    let file = File::open(&comment_tokens_path)?;
    let reader = BufReader::new(file);

    trace!("Parsing JSON from '{}'", comment_tokens_path.display());
    let data: serde_json::Value = match serde_json::from_reader(reader) {
        Ok(value) => value,
        Err(e) => {
            error!(
                "Failed to parse JSON from '{}': {}. Defaulting comment token to '#'.",
                comment_tokens_path.display(),
                e
            );
            // Return default instead of erroring out completely
            return Ok("#".to_string());
            // Or return Err(e.into()); if parsing failure should stop the process
        }
    };

    let languages_map = data
        .as_object()
        .ok_or_else(|| {
            error!(
                "JSON data in '{}' is not a top-level object. Defaulting comment token to '#'.",
                comment_tokens_path.display()
            );
            FileProcessingError::Msg(format!(
                "Invalid JSON format in '{}'",
                comment_tokens_path.display()
            ))
        })
        .unwrap();

    trace!(
        "Searching for extension '{}' in parsed JSON data.",
        extension
    );
    // Iterate through the language definitions in the JSON
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
                    // Extension matches, now find the 'comment_token'
                    if let Some(token_value) = language_details.get("comment_token") {
                        if let Some(token_str) = token_value.as_str() {
                            debug!(
                                "Found comment token '{}' for extension '{}'.",
                                token_str, extension
                            );
                            return Ok(token_str.to_string());
                        } else {
                            warn!(
                                "'comment_token' found for extension '{}' but is not a string. Defaulting to '#'.",
                                extension
                            );
                        }
                    } else {
                        warn!(
                            "Extension '{}' matched, but no 'comment_token' field found for its language entry. Defaulting to '#'.",
                            extension
                        );
                    }
                    // If extension matched but token wasn't found or valid, default
                    return Ok("#".to_string());
                }
            }
        }
    }

    // If no matching language/extension was found after checking all entries
    debug!(
        "Extension '{}' not found in comment tokens file '{}'. Defaulting comment token to '#'.",
        extension,
        comment_tokens_path.display()
    );
    Ok("#".to_string())
}

/// Formats the raw license header text by prepending the comment character to each line.
///
/// # Arguments
///
/// * `header_content`: The raw license header text.
/// * `comment_token`: The line comment token (e.g., "//", "#").
///
/// # Returns
///
/// A `String` containing the formatted header, ready to be prepended.
fn format_header_with_comments(
    header_content: &str,
    comment_token: &str,
    seperator: char,
) -> String {
    trace!("Formatting header with comment token: '{}'", comment_token);
    let mut formatted_header = String::new();
    let lines: Vec<&str> = header_content.trim_end().split('\n').collect(); // Trim trailing newline before splitting
    let line_count = lines.len();

    for (i, line) in lines.iter().enumerate() {
        formatted_header.push_str(comment_token);
        formatted_header.push(' '); // Add space after comment token
        formatted_header.push_str(line);
        if i < line_count - 1 {
            formatted_header.push('\n');
        } else {
            formatted_header.push(seperator);
            formatted_header.push_str("\n");
        }
        trace!(
            "Formatted line {}: {}",
            i + 1,
            formatted_header.lines().last().unwrap_or("")
        );
    }

    debug!(
        "Header formatting complete. Result has {} lines.",
        line_count
    );
    formatted_header // The final marker character is added in prepend_header_to_file
}

// This function seems unused in the current flow, keeping it commented out for now.
// Re-enable if needed for markdown metadata stripping elsewhere.
/*
fn strip_metadata(data: Vec<char>) -> String {
    trace!("Attempting to strip YAML front matter (---).");
    // ... (original implementation) ...
    result
}
*/

/// Prepends the formatted license header (and a marker) to the specified file.
/// Uses a temporary file and atomic rename for safety. Handles shebangs correctly.
///
/// # Arguments
///
/// * `formatted_header`: The license header, already formatted with comment tokens and newlines.
/// * `header_marker`: The character to append after the header as a marker (e.g., SOT char).
/// * `file_path`: The path to the file to modify.
///
/// # Returns
///
/// An `io::Result<()>` indicating success or failure.
fn prepend_header_to_file<P: AsRef<Path>>(formatted_header: &str, file_path: P) -> io::Result<()> {
    let file_path = file_path.as_ref();
    debug!("Prepending header to file: '{}'", file_path.display());

    // Create a temporary file in the same directory to ensure atomic rename works across devices.
    let parent_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
    trace!(
        "Creating temporary file in directory: '{}'",
        parent_dir.display()
    );
    let mut temp_file = NamedTempFile::new_in(parent_dir)?;
    let temp_path_display = temp_file.path().display().to_string(); // Capture path for logging before potential close
    debug!("Temporary file created: '{}'", temp_path_display);

    // Read the original file content
    trace!("Reading original content from '{}'", file_path.display());
    let mut original_content = Vec::new();
    File::open(file_path)?.read_to_end(&mut original_content)?;
    trace!("Read {} bytes from original file.", original_content.len());

    // --- Handle Shebang ---
    let mut shebang_line: Option<&[u8]> = None;
    let mut content_after_shebang: &[u8] = &original_content;

    if original_content.starts_with(b"#!") {
        debug!("Shebang detected in '{}'", file_path.display());
        // Find the first newline character
        if let Some(newline_pos) = original_content.iter().position(|&c| c == b'\n') {
            shebang_line = Some(&original_content[0..=newline_pos]); // Include the newline
            content_after_shebang = &original_content[newline_pos + 1..];
            trace!(
                "Extracted shebang ({} bytes). Remaining content starts after position {}.",
                shebang_line.unwrap().len(),
                newline_pos
            );
        } else {
            // Shebang detected but no newline? Treat the whole file as the shebang line.
            warn!(
                "Shebang detected in '{}' but no newline found. Treating entire file as shebang.",
                file_path.display()
            );
            shebang_line = Some(&original_content);
            content_after_shebang = &[]; // No content after shebang
        }
    } else {
        trace!("No shebang detected in '{}'.", file_path.display());
    }
    // ---

    // --- Write to Temporary File ---
    trace!("Writing content to temporary file '{}'", temp_path_display);

    // 1. Write Shebang (if present)
    if let Some(shebang) = shebang_line {
        trace!("Writing shebang to temp file.");
        temp_file.write_all(shebang)?;
        // Add an extra padding newline for separation
        temp_file.write_all(b"\n")?;
    }

    // 2. Write Formatted License Header
    trace!("Writing formatted license header to temp file.");
    temp_file.write_all(formatted_header.as_bytes())?;

    // 3. Write Separator Newline after marker (important!)
    temp_file.write_all(b"\n")?;

    // 4. Write the rest of the original content
    trace!(
        "Writing remaining original content ({} bytes) to temp file.",
        content_after_shebang.len()
    );
    temp_file.write_all(content_after_shebang)?;

    trace!(
        "Finished writing to temporary file '{}'.",
        temp_path_display
    );
    // ---

    // --- Atomically Replace Original File ---
    debug!(
        "Persisting temporary file '{}' over original file '{}'",
        temp_path_display,
        file_path.display()
    );
    // tempfile::persist handles the atomic rename/replace operation.
    if let Err(persist_error) = temp_file.persist(file_path) {
        error!(
            "Failed to persist temporary file '{}' to '{}': {}",
            temp_path_display,
            file_path.display(),
            persist_error.error
        );
        // The temporary file might still exist, drop it to attempt deletion.
        drop(persist_error.file); // Explicitly drop the file from the error
        return Err(persist_error.error); // Return the original IO error
    }
    // ---

    debug!(
        "Successfully prepended header and replaced original file '{}'",
        file_path.display()
    );
    Ok(())
}

/// Handles the `init` command logic (currently a placeholder).
fn run_init(args: InitArgs) -> Result<(), FileProcessingError> {
    debug!("Starting run_init with args: {:?}", args);

    let project_root = match args.path {
        Some(p) => {
            debug!("Using specified path for initialization: '{}'", p.display());
            p
        }
        None => {
            let cwd = std::env::current_dir()?;
            debug!(
                "No path specified, using current working directory: '{}'",
                cwd.display()
            );
            cwd
        }
    };

    info!(
        "Initializing configuration (placeholder) in: '{}'",
        project_root.display()
    );

    // --- Configuration File Generation Logic ---
    // TODO: Implement config file generation.
    //       - Determine actual config file path (e.g., project_root/.licenserc or XDG path).
    //       - Define default configuration values (maybe embed or use a template file).
    //       - Consider existing CLI options/arguments if relevant for defaults.
    //       - Write the default configuration file.
    let config_file_path = project_root.join(".lichenrc"); // Example path
    warn!("Configuration file generation is not yet implemented.");
    println!(
        "Placeholder: Would generate default config file, potentially at '{}'",
        config_file_path.display()
    );
    // ---

    Ok(())
}
