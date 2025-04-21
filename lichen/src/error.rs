//! # Error Handling
//!
//! Defines a single “mega” error type for the Lichen application.

use handlebars;
use regex;
use serde_json;
use std::{error::Error, fmt, io};
use walkdir;

/// All errors that can occur in the Lichen application, including
/// general, file-processing, and template-rendering errors.
#[derive(Debug)]
pub enum LichenError {
    /// An invalid index was given.
    InvalidIndex(usize),

    /// A required license is missing.
    MissingLicense,

    /// An invalid filesystem path was encountered.
    InvalidPath(String),

    /// Could not determine project directories.
    ProjectDirsError(String),

    /// An I/O error occurred.
    IoError(io::Error),

    /// An error occurred while walking a directory.
    WalkdirError(walkdir::Error),

    /// An error occurred while handling a regex pattern
    RegexError(String, regex::Error),

    /// Failed to parse JSON data.
    JsonError(serde_json::Error),

    /// An error occurred during template rendering.
    RenderError(handlebars::RenderError),

    /// Generic error message.
    Msg(String),
}

impl fmt::Display for LichenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LichenError::InvalidIndex(idx) => write!(f, "Invalid index: {}", idx),
            LichenError::MissingLicense => write!(
                f,
                "Missing license: A license must be set either via `lichen gen <LICENSE_ID>` or in config (lichen.toml)"
            ),
            LichenError::RegexError(pattern, err) => {
                write!(f, "Regex Error: for this pattern {} {}", pattern, err)
            }
            LichenError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            LichenError::ProjectDirsError(err) => write!(f, "Project directory error: {}", err),
            LichenError::IoError(err) => write!(f, "IO error: {}", err),
            LichenError::WalkdirError(err) => write!(f, "Directory walk error: {}", err),
            LichenError::JsonError(err) => write!(f, "JSON error: {}", err),
            LichenError::RenderError(err) => write!(f, "Template rendering error: {}", err),
            LichenError::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for LichenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LichenError::IoError(err) => Some(err),
            LichenError::WalkdirError(err) => Some(err),
            LichenError::JsonError(err) => Some(err),
            LichenError::RenderError(err) => Some(err),
            _ => None,
        }
    }
}

// Conversion from underlying errors to LichenError for `?` support.
impl From<io::Error> for LichenError {
    fn from(err: io::Error) -> Self {
        LichenError::IoError(err)
    }
}

impl From<walkdir::Error> for LichenError {
    fn from(err: walkdir::Error) -> Self {
        LichenError::WalkdirError(err)
    }
}

impl From<serde_json::Error> for LichenError {
    fn from(err: serde_json::Error) -> Self {
        LichenError::JsonError(err)
    }
}

impl From<handlebars::RenderError> for LichenError {
    fn from(err: handlebars::RenderError) -> Self {
        LichenError::RenderError(err)
    }
}

impl From<&str> for LichenError {
    fn from(msg: &str) -> Self {
        LichenError::Msg(msg.to_string())
    }
}

impl From<String> for LichenError {
    fn from(msg: String) -> Self {
        LichenError::Msg(msg)
    }
}
