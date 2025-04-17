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
use clap::{Args, ColorChoice, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, TraceLevel, Verbosity};
use directories::ProjectDirs;
use futures::stream::{self, StreamExt};
use jiff::civil::Date;
use log::{debug, error, info, trace, warn}; // Import specific log levels
use regex::Regex;
use serde_json;
use std::sync::Arc;
use tempfile::NamedTempFile;
use walkdir::{self, WalkDir};

// Local module imports
mod license;
use license::License;

// ▰▰▰ Custom Error Types ▰▰▰ //

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
    Msg(String),
}

// ▰▰▰ Error Trait Implementations ▰▰▰ //

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

// ▰▰▰ Error Conversion Implementations ▰▰▰ //

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

// ▰▰▰ CLI Argument Structs ▰▰▰ //

fn parse_year_to_date(s: &str) -> Result<Date, String> {
    // Parse the provided string as a signed 16‑bit year
    let year: i16 = s
        .parse()
        .map_err(|e| format!("invalid year `{}`: {}", s, e))?;
    // Build a date from the Janurary first of that year (Happy new years!)
    Date::new(year, 1, 1).map_err(|e| format!("invalid date: {}", e))
}

use clap::builder::styling;
const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());
/// A license management cli tool
#[derive(Parser, Debug)]
#[command(author, version, about, styles = STYLES, long_about = None, color = ColorChoice::Auto)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: Verbosity<TraceLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a license file (e.g., LICENSE or LICENSE.md)
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
    #[arg(short, long, value_parser = parse_year_to_date)]
    year: Option<Date>,
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

    /// When applying headers, which kind of comment token the user *wants*
    /// Completely possible line or block doesn't exist, in which case it falls back to the other.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    prefer_block: bool,

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

// ▰▰▰ Custom Structs ▰▰▰ //

#[derive(Debug, Clone)]
enum CommentToken {
    Line(String),
    Block { start: String, end: String },
}

impl CommentToken {
    // Bootstrap off of into string functions to fill fields
    fn line(tok: impl Into<String>) -> Self {
        CommentToken::Line(tok.into())
    }

    fn block(start: impl Into<String>, end: impl Into<String>) -> Self {
        CommentToken::Block {
            start: start.into(),
            end: end.into(),
        }
    }
}

// ▰▰▰ Main application logic ▰▰▰ //

#[tokio::main]
async fn main() -> ExitCode {
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
            run_apply(args).await
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

// ▰▰▰ Helper functions ▰▰▰ //

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
    let year = args.year.unwrap_or_else(|| jiff::Zoned::now().date()); // Fallback to todays date
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

    // ▰▰▰ Output File Generation ▰▰▰ //
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
async fn run_apply(args: ApplyArgs) -> Result<(), FileProcessingError> {
    debug!("Starting run_apply with args: {:?}", args);

    // --- Parameter Resolution (CLI vs. Config - Placeholder) ---
    // TODO: Load license, exclude_pattern, etc. from config if not in args.
    let license = args
        .license
        .ok_or("License SPDX ID is required via CLI or config for 'apply' command")?;
    let exclude_pattern = &args.exclude;
    let targets = &args.target;
    let preference = args.prefer_block;
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
        apply_headers_to_files(&license_header_content, &files_to_process, 50, preference).await?;
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
/// * `max_concurrency` A usize that declares the number of files to work on in concert
///
/// # Returns
///
/// An `io::Result<()>` indicating success or failure.
pub async fn apply_headers_to_files(
    header_content: &str,
    paths: &[PathBuf],
    max_concurrency: usize,
    prefers_block: bool,
) -> Result<(), FileProcessingError> {
    use std::sync::Arc;
    use tokio::fs;
    debug!("Starting to apply headers to {} files.", paths.len());

    // Marker for end of header (SOT)
    const HEADER_MARKER: char = '\x02';

    // Share header content safely across tasks
    let header_content = Arc::new(header_content.to_string());

    let results = stream::iter(paths.to_owned())
        .map(|path| {
            let header_content = header_content.clone();
            async move {
                let mut applied = 0;
                let mut skipped = 0;
                let mut errors = 0;

                debug!("Processing file: '{}'", path.display());

                // |1| Directories cannot be written to skip.
                if path.is_dir() {
                    warn!("Skipping directory: '{}'", path.display());
                    skipped += 1;
                    return (applied, skipped, errors);
                }

                // |2| Read file content as string, to string.
                let content = match fs::read_to_string(&path).await {
                    Ok(c) => c,
                    Err(e) => {
                        error!("Failed to read '{}': {}. Skipping.", path.display(), e);
                        errors += 1;
                        return (applied, skipped, errors);
                    }
                };

                // |3| Skip if header marker already present
                if content.contains(HEADER_MARKER) {
                    info!(
                        "Already contains header marker, skipping '{}'",
                        path.display()
                    );
                    skipped += 1;
                    return (applied, skipped, errors);
                }

                // |4| Find comment token for extention
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let comment_char = match get_comment_char(ext) {
                    Ok(tok) => tok,
                    Err(e) => {
                        error!(
                            "No comment token for '{}': {}. Skipping.",
                            path.display(),
                            e
                        );
                        errors += 1;
                        return (applied, skipped, errors);
                    }
                };

                // |5| Header formatting
                let formatted = format_header_with_comments(
                    &header_content,
                    &comment_char,
                    prefers_block,
                    HEADER_MARKER,
                );

                // |6| Shebang handling
                let (shebang, rest) = if content.starts_with("#!") {
                    if let Some(pos) = content.find('\n') {
                        let (sb, rem) = content.split_at(pos + 1);
                        (Some(sb), rem)
                    } else {
                        // The whole file is a shebang??
                        (Some(content.as_str()), "")
                    }
                } else {
                    (None, content.as_str())
                };

                // |7| Create buffer for text and write to it
                let mut new_text = String::with_capacity(
                    shebang.map_or(0, |s| s.len()) +
                    formatted.len() +
                    1 + // newline after header
                    rest.len(),
                );
                if let Some(sb) = shebang {
                    new_text.push_str(sb);
                    // ENSURE a blank line after shebang (pretty!)
                    new_text.push('\n');
                }
                new_text.push_str(&formatted);

                // Separate the header from the rest of the code
                new_text.push('\n');
                new_text.push_str(rest);

                // |8| Write the rest
                match fs::write(&path, new_text).await {
                    Ok(_) => {
                        info!("Applied header to '{}'", path.display());
                        applied += 1;
                    }
                    Err(e) => {
                        error!("Failed to write header to '{}': {}", path.display(), e);
                        errors += 1;
                    }
                }

                (applied, skipped, errors)
            }
        })
        .buffer_unordered(max_concurrency)
        .collect::<Vec<_>>()
        .await;

    // Aggregate results
    let (applied, skipped, errors) = results
        .into_iter()
        .fold((0usize, 0usize, 0usize), |(a0, s0, e0), (a1, s1, e1)| {
            (a0 + a1, s0 + s1, e0 + e1)
        });

    info!(
        "Summary: {} applied, {} skipped, {} errors.",
        applied, skipped, errors
    );

    if errors > 0 {
        Err(FileProcessingError::Msg(format!(
            "Encountered {} errors during header application.",
            errors
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
fn get_comment_char(extension: &str) -> Result<Vec<CommentToken>, FileProcessingError> {
    trace!(
        "Looking up comment character for extension: '{}'",
        extension
    );
    let comment_tokens_path = get_comment_tokens_path()?;
    // collect all tokens here
    let mut tokens = Vec::new();

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
            return Err(FileProcessingError::InvalidPath(
                "In the languages JSON".to_string(),
            ));
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
                            // Rip out the expected keys
                            let start = obj
                                .get("start")
                                .and_then(|v| v.as_str())
                                .expect("`block_comment_tokens.start` must be a string")
                                .to_string();
                            let end = obj
                                .get("end")
                                .and_then(|v| v.as_str())
                                .expect("`block_comment_tokens.end` must be a string")
                                .to_string();

                            // Have the start/end pair
                            debug!(
                                "Block comments start with `{}` and end with `{}`",
                                start, end
                            );

                            tokens.push(CommentToken::Block { start, end });
                        } else {
                            warn!("`block_comment_tokens` is not an object");
                        }
                    }
                    return Ok(tokens);
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
    tokens.push(CommentToken::Line(String::from("#")));
    return Ok(tokens);
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
    comment_tokens: &Vec<CommentToken>,
    prefers_block: bool,
    seperator: char,
) -> String {
    trace!(
        "Determing comment token from these options: '{:?}'",
        comment_tokens
    );

    // Attempt to find preferred variant
    let comment_token = comment_tokens
        .iter()
        .find(|ct| {
            if prefers_block {
                matches!(ct, CommentToken::Block { .. })
            } else {
                matches!(ct, CommentToken::Line(_))
            }
        })
        .or_else(|| {
            // Falback if preferred not found
            comment_tokens.iter().find(|ct| {
                if prefers_block {
                    matches!(ct, CommentToken::Line(_))
                } else {
                    matches!(ct, CommentToken::Block { .. })
                }
            })
        })
        .unwrap();

    trace!(
        "Formatting header with comment token: '{:?}'",
        comment_token
    );

    let mut formatted_header = String::new();

    match comment_token {
        CommentToken::Line(tok) => {
            let lines: Vec<&str> = header_content.trim_end().split('\n').collect(); // Trim trailing newline before splitting
            let line_count = lines.len();

            for (i, line) in lines.iter().enumerate() {
                formatted_header.push_str(&tok);
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
        }
        CommentToken::Block { start, end } => {
            formatted_header.push_str(&start);
            formatted_header.push('\n'); // You want some distance between block comments
            formatted_header.push_str(header_content);
            formatted_header.push('\n'); // You want some distance between block comments
            formatted_header.push_str(&end);
            formatted_header.push(seperator); // Mark
            formatted_header.push('\n'); // Padding
        }
    }

    debug!("Header formatting complete");
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
