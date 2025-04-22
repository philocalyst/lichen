// src/error.rs
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Directory traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("Failed to create output directory '{path}': {source}")]
    CreateDir {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Invalid input path: {0}")]
    InvalidInputPath(PathBuf),

    #[error("Invalid output path: {0}")]
    InvalidOutputPath(PathBuf),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("HTML parsing error: {0}")]
    HtmlParsing(String), // html_parser::Error doesn't impl std::error::Error

    #[error("Failed to process file '{path}': {message}")]
    FileProcessing { path: PathBuf, message: String },

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(PathBuf),
}

// Helper to convert html_parser::Error
impl From<html_parser::Error> for AppError {
    fn from(err: html_parser::Error) -> Self {
        AppError::HtmlParsing(err.to_string())
    }
}
