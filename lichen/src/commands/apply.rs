//! # Apply Command
//!
//! Logic for the `lichen apply` command.

use crate::cli::ApplyArgs;
use crate::error::FileProcessingError;
use crate::license::License; // Ensure License is imported
use crate::paths;
use crate::utils;
use log::{debug, error, info, trace, warn};
use std::fs;
use std::io;

/// Handles the `apply` command logic.
pub async fn handle_apply(args: ApplyArgs) -> Result<(), FileProcessingError> {
    debug!("Starting handle_apply with args: {:?}", args);

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
    } else {
        // TODO: Implement non-in-place logic
        error!("Non-in-place application is not yet implemented.");
        return Err(FileProcessingError::Msg(
            "Non-in-place application not implemented.".to_string(),
        ));
    }
    info!("Exclusion pattern: {:?}", exclude_pattern);
    info!("Prefer block comments: {}", preference);

    // --- Get License Header Content ---
    // Headers often use a specific template, e.g., "header.txt" or just "txt"
    // Let's assume "header.txt" first, then fallback to "txt"
    let header_template_path = match paths::get_license_path(&license, "header.txt") {
        Ok(path) if path.exists() => path,
        _ => {
            debug!("No 'header.txt' found, falling back to 'txt' for header content.");
            paths::get_license_path(&license, "txt")?
        }
    };

    if !header_template_path.exists() {
        error!(
            "License header template file not found at '{}' (tried .header.txt and .txt).",
            header_template_path.display()
        );
        return Err(FileProcessingError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "License header template not found for {}: {}",
                license.spdx_id(),
                header_template_path.display()
            ),
        )));
    }

    debug!(
        "Reading license header content from: '{}'",
        header_template_path.display()
    );
    let license_header_content = fs::read_to_string(&header_template_path)?;
    trace!(
        "License header content read successfully:\n{}",
        license_header_content
    );
    // ---

    // --- Find Files ---
    let files_to_process = utils::get_valid_files(targets, exclude_pattern)?;
    if files_to_process.is_empty() {
        info!(
            "No files require processing based on targets and exclusions. Exiting 'apply' command."
        );
        return Ok(()); // Nothing to do
    }
    // ---

    // --- Apply Headers ---
    // Currently only supports in-place
    if in_place {
        info!(
            "Applying license headers in-place to {} files...",
            files_to_process.len()
        );
        // TODO: Make concurrency configurable?
        let max_concurrency = num_cpus::get().max(1); // Use available cores
        utils::apply_headers_to_files(
            &license_header_content,
            &files_to_process,
            max_concurrency,
            preference,
        )
        .await?;
    }
    // Non-in-place logic would go in an `else` block here.
    // ---

    info!("Finished applying license headers.");
    Ok(())
}
