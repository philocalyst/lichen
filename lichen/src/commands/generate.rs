//! # Generate Command
//!
//! Logic for the `lichen gen` command.

use crate::cli::GenArgs;
use crate::config::{Authors, Config};
use crate::error::LichenError;
use crate::license::License; // Make sure License is imported
use crate::paths;
use crate::utils;
use jiff::civil::Date;
use log::{debug, error, info, trace, warn};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GenSettings {
    pub license: License,
    pub multiple: bool,
    pub targets: Vec<PathBuf>,
    pub authors: Option<Authors>,
    pub date: Date,
}

impl GenSettings {
    pub fn new(cli: &GenArgs, cfg: &Config, index: Option<usize>) -> Result<Self, LichenError> {
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

        let multiple = cli
            .multiple
            .or_else(|| cfg.multiple)
            .unwrap_or_else(|| false);

        Ok(GenSettings {
            license,
            targets,
            authors,
            date,
            multiple,
        })
    }
}

/// Handles the `gen` command logic.
pub fn handle_gen(settings: &GenSettings) -> Result<(), LichenError> {
    debug!("Starting handle_gen with args: {:?}", settings);

    // --- Parameter Resolution (CLI vs. Config - Placeholder) ---
    // TODO: Load license, author, year from config if not provided in args.
    let license = settings.license;
    let targets = &settings.targets;
    let multiple = settings.multiple;
    let authors = &settings.authors;
    let year = settings.date;
    let template_extension = "template.txt"; // Base template extension
    let output_extension = "txt"; // Default output extension
    // let output_extension = if args.markdown { "md" } else { "txt" };
    // ---

    info!(
        "Generating license file for: {}, Year: {}, Authors: {:?}, Format: {}",
        license.spdx_id(),
        year.year(), // Log only the year for simplicity
        authors,
        output_extension
    );

    // --- Template Fetching ---
    let license_template_path = paths::get_license_path(&license, template_extension)?;
    if !license_template_path.exists() {
        error!(
            "License template file not found at '{}'. Ensure templates are installed correctly.",
            license_template_path.display()
        );
        return Err(LichenError::IoError(io::Error::new(
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
        .map_err(LichenError::RenderError)?; // Convert RenderError
    debug!("License content rendered successfully.");
    trace!("Rendered content:\n{}", rendered_license);

    for target in targets {
        if target.is_file() {
            warn!(
                "Skipped target \"{}\", as generate can not place a license in a file, please look at the apply subcommand for this usage",
                target.to_string_lossy()
            );
            continue;
        }
        let mut output_filename;
        if multiple {
            output_filename = target.join(license.spdx_id().to_string() + "_" + "LICENSE");
        } else {
            output_filename = target.join("LICENSE");
        }

        // If extention is not txt, add it. Otherwise, the paradigm is to have it without.
        if output_extension != "txt" {
            output_filename.set_extension(output_extension);
        }

        fs::write(&output_filename, &rendered_license)?;
        info!("License file written to '{}'", output_filename.display());
    }
    // ---

    Ok(())
}
