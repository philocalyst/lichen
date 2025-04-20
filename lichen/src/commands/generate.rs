//! # Generate Command
//!
//! Logic for the `lichen gen` command.

use crate::cli::GenArgs;
use crate::error::FileProcessingError;
use crate::license::License; // Make sure License is imported
use crate::paths;
use crate::utils;
use jiff;
use log::{debug, error, info, trace, warn};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Handles the `gen` command logic.
pub fn handle_gen(args: GenArgs) -> Result<(), FileProcessingError> {
    debug!("Starting handle_gen with args: {:?}", args);

    // --- Parameter Resolution (CLI vs. Config - Placeholder) ---
    // TODO: Load license, author, year from config if not provided in args.
    let license = args
        .license
        .ok_or("License SPDX ID is required via CLI or config for 'gen' command")?;
    let authors = args.authors.unwrap_or_else(|| {
        // TODO: Get default author from config or environment?
        warn!("No authors specified, using placeholder.");
        vec!["Your Name".to_string()] // Placeholder
    });
    let year = args.date.unwrap_or_else(|| jiff::Zoned::now().date()); // Fallback to today's date
    let template_extension = "template.txt"; // Base template extension
    let output_extension = "txt"; // Default output extension
    // let output_extension = if args.markdown { "md" } else { "txt" };
    // ---

    info!(
        "Generating license file for: {}, Year: {}, Authors: {}, Format: {}",
        license.spdx_id(),
        year.year(), // Log only the year for simplicity
        authors.join(", "),
        output_extension
    );

    // --- Template Fetching ---
    let license_template_path = paths::get_license_path(&license, template_extension)?;
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

    let template_content = fs::read_to_string(&license_template_path)?;
    debug!("License template content read successfully.");
    trace!("Template content:\n{}", template_content); // Trace for full content

    // --- Render Template ---
    let rendered_license = utils::render_license(&template_content, &year, &authors)
        .map_err(FileProcessingError::RenderError)?; // Convert RenderError
    debug!("License content rendered successfully.");
    trace!("Rendered content:\n{}", rendered_license);

    // --- Output File Generation ---
    let mut output_filename = PathBuf::from("LICENSE");
    // Only add extension for non-txt formats or if explicitly requested
    if output_extension != "txt" {
        output_filename.set_extension(output_extension);
    }
    debug!(
        "Determined output filename: '{}'",
        output_filename.display()
    );

    fs::write(&output_filename, rendered_license)?;
    info!("License file written to '{}'", output_filename.display());
    // ---

    Ok(())
}
