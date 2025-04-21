//! # Apply Command
//!
//! Logic for the `lichen apply` command.

use crate::cli::ApplyArgs;
use crate::config::{Authors, Config};
use crate::error::LichenError;
use crate::license::License; // Ensure License is imported
use crate::paths;
use crate::utils;
use jiff::civil::Date;
use log::{debug, error, info, trace, warn};
use regex::Regex;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ApplySettings {
    pub license: License,
    pub in_place: bool,
    pub prefer_block: bool,
    pub multiple: bool,
    pub authors: Option<Authors>,
    pub ignore_git_ignore: bool,
    pub exclude: Option<Regex>,
    pub targets: Vec<PathBuf>,
    pub date: Date,
}

fn load_gitignore_patterns() -> Vec<String> {
    // e.g. walk with the `ignore` crate or
    // read & parse your .gitignore file(s)
    vec!["target/.*".into(), r"\.DS_Store".into()]
}

fn build_exclude_regex(
    cli: &ApplyArgs,
    cfg: &Config,
    with_gitignore: bool,
    index: Option<usize>,
) -> Option<Regex> {
    // 1) collect all raw pattern strings
    let mut pats = Vec::new();

    // a) .gitignore patterns (unless disabled)
    if !with_gitignore {
        pats.extend(load_gitignore_patterns());
    }

    // b) global exclude from config
    if let Some(globs) = cfg.exclude.as_ref() {
        for re in globs.iter() {
            pats.push(re.as_str().to_string());
        }
    }

    // c) per‑license exclude
    if let Some(i) = index {
        if let Some(lic) = cfg.licenses.as_ref()?.get(i) {
            if let Some(exc) = lic.exclude.as_ref() {
                pats.push(exc.to_string());
            }
        }
    }

    // d) CLI override
    if let Some(cli_exc) = cli.exclude.as_ref() {
        pats.push(cli_exc.to_string());
    }

    // nothing to exclude?
    if pats.is_empty() {
        return None;
    }

    // 2) wrap each in a non‑capturing group and join with |
    let alternation = pats
        .into_iter()
        .map(|p| format!("(?:{})", p))
        .collect::<Vec<_>>()
        .join("|");

    // 3) compile once
    match Regex::new(&alternation) {
        Ok(re) => Some(re),
        Err(err) => {
            eprintln!(
                "error: failed to compile combined exclude regex `{}`: {}",
                alternation, err
            );
            None
        }
    }
}

impl ApplySettings {
    pub fn new(cli: &ApplyArgs, cfg: &Config, index: Option<usize>) -> Result<Self, LichenError> {
        let license = if let Some(cli_lic) = cli.license.clone() {
            // user explicitly passed one on the command line
            cli_lic
        } else if let Some(idx) = index {
            // user did `lichen gen` without `--license` but we have a config entry
            let lic = cfg
                .licenses
                .as_ref()
                .expect("If an index is passed, assume there is a license")
                .get(idx)
                .ok_or(LichenError::InvalidIndex(idx))?;
            lic.id.clone()
        } else {
            // no CLI value, no config entry
            return Err(LichenError::MissingLicense);
        };

        let default_target = vec![PathBuf::from(".")];

        let targets: Vec<PathBuf> = if let Some(cli_targets) = cli.targets.clone() {
            // user passed authors on the command line
            cli_targets
        } else if let Some(idx) = index {
            // fall back to config’s optional authors
            cfg.licenses
                .as_ref()
                .expect("If an index is passed, assume there is a license")
                .get(idx)
                .and_then(|lic| lic.targets.clone())
                .unwrap_or_else(|| default_target)
        } else {
            // Falling back on target "." (Current directory)
            default_target
        };

        let authors: Option<Authors> = if let Some(cli_authors) = cli.authors.clone() {
            // user passed authors on the command line
            Some(cli_authors)
        } else if let Some(idx) = index {
            // fall back to config’s optional authors
            cfg.licenses
                .as_ref()
                .expect("If an index is passed, assume there is a license")
                .get(idx)
                .and_then(|lic| lic.authors.clone())
        } else {
            // no CLI, no config, no author.
            None
        };

        let date = if let Some(cli_date) = cli.date {
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

        let ignore_git_ignore = cli
            .ignore_git_ignore
            .or_else(|| cfg.ignore_git_ignore)
            .unwrap_or_else(|| false);

        let exclude = build_exclude_regex(&cli, &cfg, ignore_git_ignore, index);

        let multiple = cli
            .multiple
            .or_else(|| cfg.multiple)
            .unwrap_or_else(|| false);

        let in_place = cli
            .in_place
            .or_else(|| cfg.in_place)
            .unwrap_or_else(|| false);

        let prefer_block = cli
            .prefer_block
            .or_else(|| cfg.prefer_block)
            .unwrap_or_else(|| false);

        Ok(ApplySettings {
            exclude,
            license,
            targets,
            prefer_block,
            in_place,
            ignore_git_ignore,
            authors,
            date,
            multiple,
        })
    }
}

/// Handles the `apply` command logic.
pub async fn handle_apply(settings: &ApplySettings) -> Result<(), LichenError> {
    debug!("Starting handle_apply with args: {:?}", settings);

    // --- Parameter Resolution (CLI vs. Config - Placeholder) ---
    // TODO: Load license, exclude_pattern, etc. from config if not in args.
    let license = settings.license;
    let exclude_pattern = &settings.exclude;
    let targets = &settings.targets;
    let authors = &settings.authors;
    let year = &settings.date;
    let preference = settings.prefer_block;
    let in_place = settings.in_place;
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
        return Err(LichenError::Msg(
            "Non-in-place application not implemented.".to_string(),
        ));
    }
    info!("Exclusion pattern: {:?}", exclude_pattern);
    info!("Prefer block comments: {}", preference);

    // --- Get License Header Content ---
    // Headers often use a specific template, e.g., "header.txt" or just "txt"
    // Let's assume "header.txt" first, then fallback to "txt"
    let header_template_path = match paths::get_license_path(&license, "template.txt") {
        Ok(path) if path.exists() => path,
        _ => {
            debug!("No 'header.txt' found, falling back to 'txt' for header content.");
            paths::get_license_path(&license, "txt")?
        }
    };

    if !header_template_path.exists() {
        error!(
            "License header template file not found at '{}' (tried .template.txt and .txt).",
            header_template_path.display()
        );
        return Err(LichenError::IoError(io::Error::new(
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
    let template_content = fs::read_to_string(&header_template_path)?;
    trace!(
        "License header content read successfully:\n{}",
        template_content
    );
    let rendered_license = utils::render_license(&template_content, &year, &authors)
        .map_err(LichenError::RenderError)?; // Convert RenderError
    debug!("License content rendered successfully.");
    trace!("Rendered content:\n{}", rendered_license);

    // ---

    // --- Find Files ---
    let files_to_process = utils::get_valid_files(targets, exclude_pattern)?;
    if files_to_process.is_empty() {
        return Err(LichenError::Msg(
            "No files require processing based on targets and exclusions. Exiting 'apply' command."
                .to_string(),
        )); // Nothing to do, error. SOMETHING needs to be done.
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
        let max_concurrency = std::thread::available_parallelism()
            .expect("There should always be some available parellism on the computer"); // Use available cores
        utils::apply_headers_to_files(
            &rendered_license,
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
