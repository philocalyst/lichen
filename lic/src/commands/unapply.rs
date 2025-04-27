use crate::cli::UnapplyArgs;
use crate::error::LichenError;
use crate::utils;
use log::{debug, info};

pub async fn handle_unapply(args: UnapplyArgs) -> Result<(), LichenError> {
    // Load options
    let targets = args.targets;
    let exclude = utils::build_exclude_regex(&args.exclude, None, args.all, None)?;

    // ▰▰▰ Find Files ▰▰▰
    let files_to_process = utils::get_valid_files(&targets, &exclude)?;
    if files_to_process.is_empty() {
        return Err(LichenError::Msg(
            "No files require processing based on targets and exclusions. Exiting 'apply' command."
                .to_string(),
        )); // Nothing to do, error. 
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
