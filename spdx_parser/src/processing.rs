// src/processing.rs
use crate::error::AppError;
use crate::parser::{parse_html_to_markdown_handlebars_v2, parse_text_template}; // Using V2
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Processes all valid files in the input directory.
/// This function is now internal to the crate.
pub(crate) fn process_directory(input_dir: &Path, output_dir: &Path) -> Result<(), AppError> {
    // Use log macros, assuming the application initializes the logger
    log::info!(
        "Processing directory '{}' to '{}'",
        input_dir.display(),
        output_dir.display()
    );

    if !input_dir.is_dir() {
        return Err(AppError::InvalidInputPath(input_dir.to_path_buf()));
    }

    // Ensure the base output directory exists before starting the walk
    fs::create_dir_all(output_dir).map_err(|e| AppError::CreateDir {
        path: output_dir.to_path_buf(),
        source: e,
    })?;
    log::trace!(
        "Ensured base output directory exists: {}",
        output_dir.display()
    );

    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok()) // Skip entries with errors
        .filter(|e| e.file_type().is_file())
    // Process only files
    {
        let input_path = entry.path();
        // Pass paths directly to process_file
        match process_file(input_path, input_dir, output_dir) {
            Ok(_) => log::info!("Successfully processed '{}'", input_path.display()),
            Err(e) => {
                // Log error and continue with other files
                log::error!("Failed to process file '{}': {}", input_path.display(), e);
            }
        }
    }

    log::info!("Finished processing directory '{}'.", input_dir.display());
    Ok(())
}

/// Processes a single file based on its extension.
/// Remains internal.
fn process_file(input_path: &Path, input_base: &Path, output_base: &Path) -> Result<(), AppError> {
    let extension = input_path
        .extension()
        .and_then(|os| os.to_str())
        .unwrap_or("");

    let content = fs::read_to_string(input_path)?;
    let processed_content: String;
    let output_filename: PathBuf;

    match extension.to_lowercase().as_str() {
        "txt" => {
            log::debug!("Processing text file: {}", input_path.display());
            processed_content = parse_text_template(&content)?;
            // Keep original name
            output_filename = input_path.file_name().unwrap().into();
        }
        "html" | "htm" => {
            log::debug!("Processing HTML file: {}", input_path.display());
            // Use the html2md based version
            processed_content = parse_html_to_markdown_handlebars_v2(&content)?;
            // Change extension to .md
            output_filename = input_path.with_extension("md").file_name().unwrap().into();
        }
        _ => {
            log::warn!("Skipping unsupported file type: {}", input_path.display());
            return Ok(());
        }
    }

    // Calculate output path, preserving directory structure
    let relative_path =
        input_path
            .strip_prefix(input_base)
            .map_err(|_| AppError::FileProcessing {
                path: input_path.to_path_buf(),
                message: format!(
                    "Failed to determine relative path based on input base '{}'",
                    input_base.display()
                ),
            })?;

    // Construct the output path: output_base / relative_path (without filename) / output_filename
    let output_path = if let Some(parent) = relative_path.parent() {
        output_base.join(parent).join(&output_filename)
    } else {
        // File is directly in input_base
        output_base.join(&output_filename)
    };

    // Ensure the specific output directory for this file exists
    if let Some(parent_dir) = output_path.parent() {
        // Check if it's different from the base output dir to avoid redundant logs/creation attempts
        if parent_dir != output_base {
            fs::create_dir_all(parent_dir).map_err(|e| AppError::CreateDir {
                path: parent_dir.to_path_buf(),
                source: e,
            })?;
            log::trace!("Ensured output directory exists: {}", parent_dir.display());
        }
    } else {
        // This case should ideally not happen if output_path is correctly formed
        log::warn!(
            "Could not determine parent directory for output path: {}",
            output_path.display()
        );
    }

    fs::write(&output_path, processed_content)?;
    log::debug!("Written processed file to {}", output_path.display());

    Ok(())
}
