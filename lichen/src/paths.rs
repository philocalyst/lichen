//! # Path Utilities
//!
//! Functions for locating application-specific directories and files.

use crate::error::FileProcessingError;
use crate::license::License;
use directories::ProjectDirs;
use log::{debug, trace};
use std::path::PathBuf;

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
pub fn get_data_dir() -> Result<PathBuf, FileProcessingError> {
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
pub fn get_license_path(
    license: &License,
    extension: &str,
) -> Result<PathBuf, FileProcessingError> {
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
pub fn get_comment_tokens_path() -> Result<PathBuf, FileProcessingError> {
    trace!("Constructing path for comment-tokens.json");
    let data_dir = get_data_dir()?;
    // Expected structure: <data_dir>/comment-tokens.json
    let path = data_dir.join("comment-tokens.json");
    debug!("Constructed comment tokens path: '{}'", path.display());
    Ok(path)
}
