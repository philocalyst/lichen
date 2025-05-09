//! # Apply Command
//!
//! Logic for the `lichen apply` command.

use crate::config::Config;
use crate::error::LichenError;
use crate::models::ApplyArgs;
use crate::models::Authors;
use crate::models::License; // Ensure License is imported
use crate::utils;
use jiff::civil::Date;
use log::{debug, info, trace};
use regex::Regex;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ApplySettings {
    pub license: License,
    pub prefer_block: bool,
    pub multiple: bool,
    pub authors: Option<Authors>,
    pub exclude: Option<Regex>,
    pub targets: Vec<PathBuf>,
    pub date: Date,
    pub dry_run: bool,
}

impl ApplySettings {
    pub fn new(cli: &ApplyArgs, cfg: &Config, index: Option<usize>) -> Result<Self, LichenError> {
        let license = if let Some(cli_lic) = cli.license_args.license {
            // An explicity passed CLI license
            cli_lic
        } else if let Some(idx) = index {
            // User did `lichen gen` without `--license` but we have a config entry
            let lic = cfg
                .licenses
                .as_ref()
                .expect("If an index is passed, assume there is a license")
                .get(idx)
                .ok_or(LichenError::InvalidIndex(idx))?;
            lic.id
        } else {
            // no CLI value, no config entry, nothing.
            return Err(LichenError::MissingLicense);
        };

        let default_target = vec![PathBuf::from(".")];

        let targets: Vec<PathBuf> = if let Some(cli_targets) = cli.file_args.targets.clone() {
            // User passed targets on the command line, max priority
            cli_targets
        } else if let Some(idx) = index {
            // fall back to the targets inscribed in config
            cfg.licenses
                .as_ref()
                .expect("If an index is passed, assume there is a license")
                .get(idx)
                .and_then(|lic| lic.targets.clone())
                .unwrap_or(default_target)
        } else {
            // Falling back on target "." (Current directory)
            default_target
        };

        let authors: Option<Authors> = if let Some(cli_authors) = cli.license_args.authors.clone() {
            // User passed authors on the command line
            Some(cli_authors)
        } else if let Some(idx) = index {
            // Fall back to config’s optional authors
            cfg.licenses
                .as_ref()
                .expect("If an index is passed, assume there is a license")
                .get(idx)
                .and_then(|lic| lic.authors.clone())
        } else {
            // no CLI, no config, no author.
            None
        };

        let date = if let Some(cli_date) = cli.license_args.date {
            cli_date
        } else if let Some(idx) = index {
            cfg.licenses
                .as_ref()
                .expect("If an index is passed, assume there is a license")
                .get(idx)
                .and_then(|lic| lic.date)
                .unwrap_or_else(|| jiff::Zoned::now().date())
        } else {
            jiff::Zoned::now().date()
        };

        let all = cli.file_args.all.or(cfg.all).unwrap_or(false);

        let exclude = utils::build_exclude_regex(&cli.file_args.exclude, Some(cfg), all, index)?;

        let multiple = cli.license_args.multiple.or(cfg.multiple).unwrap_or(false);

        let prefer_block = cli.prefer_block.or(cfg.prefer_block).unwrap_or(false);

        let dry_run = cli.dry_run.unwrap_or(false);

        Ok(ApplySettings {
            exclude,
            license,
            dry_run,
            targets,
            prefer_block,
            authors,
            date,
            multiple,
        })
    }
}

/// Handles the `apply` command logic.
pub async fn handle_apply(settings: &ApplySettings) -> Result<(), LichenError> {
    debug!("Starting handle_apply with args: {:?}", settings);

    // ▰▰▰ Get options from setting struct ▰▰▰
    let license = settings.license;
    let exclude_pattern = &settings.exclude;
    let targets = &settings.targets;
    let multiple = settings.multiple;
    let authors = &settings.authors;
    let year = &settings.date;
    let dry_run = settings.dry_run;
    let preference = settings.prefer_block;
    //

    debug!(
        "Applying license header for: {} to targets: {:?}",
        license.spdx_id(),
        targets
    );
    debug!("Exclusion pattern: {:?}", exclude_pattern);
    debug!("Block comments preferred?: {}", preference);

    // ▰▰▰ Get License Header Content ▰▰▰ //
    let template_content = settings.license.template_content();
    debug!(
        "Using embedded template content for {}",
        settings.license.spdx_id()
    );
    debug!("Embedded template content:\n{}", template_content);

    let rendered_license =
        utils::render_license(template_content, year, authors).map_err(LichenError::RenderError)?; // Convert RenderError for compatibility
    trace!("License content rendered successfully.");
    debug!("Rendered content:\n{}", rendered_license);

    // ▰▰▰ Find Files ▰▰▰
    let files_to_process = utils::get_valid_files(targets, exclude_pattern)?;
    if files_to_process.is_empty() {
        return Err(LichenError::Msg(
            "No files require processing based on targets and exclusions. Exiting 'apply' command."
                .to_string(),
        )); // Nothing to do, error. 
    }

    if dry_run {
        info!("Changes will impact {:?}", files_to_process);
        return Ok(());
    }

    // ▰▰▰ Apply Headers ▰▰▰
    // TODO: Make concurrency configurable?
    let max_concurrency = std::thread::available_parallelism()
        .expect("There should always be some available parellism on the computer"); // Use available cores
    utils::apply_headers_to_files(
        &rendered_license,
        &files_to_process,
        max_concurrency,
        preference,
        multiple,
    )
    .await?;

    info!("Finished applying license headers.");
    Ok(())
}
