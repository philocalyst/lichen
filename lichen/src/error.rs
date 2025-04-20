//! # Error Handling
//!
//! Defines custom error types for the Lichen application.

use std::{error::Error, fmt};

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
