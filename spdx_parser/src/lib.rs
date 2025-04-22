//! # Template Converter Library
//!
//! Provides functionality to convert custom text and HTML templates
//! into Handlebars format, with HTML being converted to Markdown.

// Declare modules. These are internal to the crate unless explicitly made public.
mod error;
mod parser;
mod processing;

// Re-export the error type so users of the library can handle errors.
pub use error::AppError;

use std::path::Path;

/// Converts template files from an input directory to an output directory.
///
/// Walks the `input_dir`, processing `.txt` and `.html`/`.htm` files found.
/// - Text files (`.txt`) are parsed for custom `<<var>>` and `<<beginOptional>>`
///   tags and converted to Handlebars syntax.
/// - HTML files (`.html`, `.htm`) are parsed, `optional-license-text` sections
///   are removed, `replaceable-license-text` sections are converted to
///   Handlebars conditionals, and the result is converted to Markdown (`.md`).
///
/// The directory structure from `input_dir` is preserved in `output_dir`.
///
/// # Arguments
///
/// * `input_dir` - The path to the directory containing the source template files.
/// * `output_dir` - The path to the directory where processed files will be written.
///   This directory will be created if it doesn't exist.
///
/// # Errors
///
/// Returns `AppError` if:
/// *   The `input_dir` is not a valid directory or is inaccessible.
/// *   There are I/O errors reading files or writing to the `output_dir`.
/// *   Directory creation fails.
/// *   Parsing of text or HTML fails.
/// *   Any other error defined in `AppError`.
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use spdx_parser::{convert_templates, AppError};
///
/// fn main() -> Result<(), AppError> {
///     // Initialize logging (optional, done by the application)
///     // env_logger::init();
///
///     let input = PathBuf::from("./my_templates");
///     let output = PathBuf::from("./processed_templates");
///
///     println!("Starting conversion from {} to {}", input.display(), output.display());
///     convert_templates(&input, &output)?;
///     println!("Conversion successful!");
///
///     Ok(())
/// }
/// ```
pub fn convert_templates(input_dir: &Path, output_dir: &Path) -> Result<(), AppError> {
    // Note: Logging should be initialized by the application using this library,
    // not the library itself. We still use log::info!, log::debug!, etc.
    log::info!(
        "Library: Starting template conversion from '{}' to '{}'",
        input_dir.display(),
        output_dir.display()
    );

    // Call the internal processing function
    processing::process_directory(input_dir, output_dir)?;

    log::info!(
        "Library: Finished template conversion for '{}'",
        input_dir.display()
    );
    Ok(())
}
