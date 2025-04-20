//! # Error Handling
//!
//! Defines custom error types for the Lichen application.

use std::{error::Error, fmt};

use handlebars;

/// Errors that can occur anywhere in the Lichen application.
#[derive(Debug)]
pub enum LichenError {
    /// An invalid index was given.
    InvalidIndex(usize),

    /// An error occurred while walking a directory.
    WalkdirError(walkdir::Error),

    /// An error occurred while walking a directory.
    MissingLicense,

    /// Failed to parse JSON data.
    JsonError(serde_json::Error),

    /// An error occurred during template rendering.
    RenderError(handlebars::RenderError),

    /// Generic error message.
    Msg(String),
}

// ▰▰▰ Display ▰▰▰ //

impl fmt::Display for LichenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LichenError::InvalidIndex(idx) => write!(f, "Invalid index: {}", idx),
            LichenError::MissingLicense => write!(
                f,
                "license must be set either via `lichen gen <ID>` or in config"
            ),
            LichenError::WalkdirError(err) => write!(f, "Directory walk error: {}", err),
            LichenError::JsonError(err) => write!(f, "JSON error: {}", err),
            LichenError::RenderError(err) => write!(f, "Template rendering error: {}", err),
            LichenError::Msg(msg) => write!(f, "{}", msg),
        }
    }
}

// ▰▰▰ std::error::Error ▰▰▰ //

impl Error for LichenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LichenError::WalkdirError(err) => Some(err),
            LichenError::JsonError(err) => Some(err),
            LichenError::RenderError(err) => Some(err),
            _ => None,
        }
    }
}

// ▰▰▰ From conversions for easy `?` support ▰▰▰ //

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

    /// An error occurred during template rendering.
    RenderError(handlebars::RenderError),

    /// Generic error message.
    Msg(String),
}

// ▰▰▰ Error Trait Implementations ▰▰▰ //

impl fmt::Display for FileProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileProcessingError::IoError(err) => write!(f, "IO error: {}", err),
            FileProcessingError::WalkdirError(err) => {
                write!(f, "Directory walk error: {}", err)
            }
            FileProcessingError::InvalidPath(path) => {
                write!(f, "Invalid path: {}", path)
            }
            FileProcessingError::JsonError(err) => {
                write!(f, "JSON processing error: {}", err)
            }
            FileProcessingError::ProjectDirsError(err) => {
                write!(f, "Project directory error: {}", err)
            }
            FileProcessingError::RenderError(err) => {
                write!(f, "Template rendering error: {}", err)
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
            FileProcessingError::RenderError(err) => Some(err),
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

impl From<handlebars::RenderError> for FileProcessingError {
    fn from(err: handlebars::RenderError) -> Self {
        FileProcessingError::RenderError(err)
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
