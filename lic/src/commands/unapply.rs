//! # Unapply Command
//!
//! Logic for the `lichen unapply` command.

use crate::error::LichenError;
use crate::models::UnapplyArgs;
use crate::utils;
use log::{debug, info};

pub async fn handle_unapply(args: UnapplyArgs) -> Result<(), LichenError> {
    // Load options
    let targets = args.file_args.targets.unwrap_or(vec![".".into()]);
    let exclude = utils::build_exclude_regex(
        &args.file_args.exclude,
        None,
        args.file_args.all.unwrap_or_default(),
        None,
    )?;

    // ▰▰▰ Find Files ▰▰▰
    let files_to_process = utils::get_valid_files(&targets, &exclude)?;
    if files_to_process.is_empty() {
        return Err(LichenError::Msg(
            "No files require processing based on targets and exclusions. Exiting 'apply' command."
                .to_string(),
        )); // Nothing to do, error. 
    }

    // If dry run was passed, just return the files changes *would* have impacted
    if args.dry_run.unwrap_or(false) {
        info!("Changes will impact {:?}", files_to_process);
        return Ok(());
    }

    // ▰▰▰ Apply Headers ▰▰▰
    // TODO: Make concurrency configurable?
    let max_concurrency = std::thread::available_parallelism()
        .expect("There should always be some available parellism on the computer"); // Use available cores
    utils::remove_headers_from_files(&files_to_process, max_concurrency).await?;

    info!(
        "Successfully unapplied license header for targets: {:?}",
        targets
    );
    debug!("Exclusion pattern: {:?}", exclude);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{FileProcessingArgs, UnapplyArgs};
    use crate::utils::{HEADER_MARKER, HEADER_MARKER_STR}; // Import marker
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn handle_unapply_dry_run_succeeds() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_unapply.rs");
        fs::write(
            &file_path,
            format!("// Header{}\n// Real Code", HEADER_MARKER_STR),
        )
        .unwrap();

        let args = UnapplyArgs {
            file_args: FileProcessingArgs {
                targets: Some(vec![file_path.clone()]),
                exclude: None,
                all: Some(true), // Ensure file is processed
            },
            dry_run: Some(true),
        };

        let result = handle_unapply(args).await;
        assert!(
            result.is_ok(),
            "handle_unapply dry run failed: {:?}",
            result
        );

        // Verify file was NOT modified in dry run
        let content = fs::read_to_string(file_path).unwrap();
        assert!(
            content.contains(HEADER_MARKER),
            "File should not be modified in dry run"
        );
    }

    // Test actual removal requires utils::remove_headers_from_files test
    // which is also somewhat integration-like due to async fs.
}
