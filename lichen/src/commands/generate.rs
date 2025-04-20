//! # Generate Command
//!
//! Logic for the `lichen gen` command.

use crate::cli::GenArgs;
use crate::config::{Author, Authors, Config};
use crate::error::{FileProcessingError, LichenError};
use crate::license::License; // Make sure License is imported
use crate::paths;
use crate::utils;
use jiff::civil::Date;
use log::{debug, error, info, trace, warn};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub struct GenSettings {
    pub license: License,
    pub multiple: bool,
    pub authors: Option<Authors>,
    pub ignore_git_ignore: bool,
    pub date: Date,
}

impl GenSettings {
    pub fn new(
        cli: &GenArgs,
        cfg: &Config,
        index: Option<usize>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let license = if let Some(cli_lic) = cli.license.clone() {
            // user explicitly passed one on the command line
            cli_lic
        } else if let Some(idx) = index {
            // user did `lichen gen` without `--license` but we have a config entry
            let lic = cfg
                .licenses
                .get(idx)
                .ok_or(LichenError::InvalidIndex(idx))?;
            lic.id.clone()
        } else {
            // no CLI value, no config entry
            return Err(Box::new(LichenError::MissingLicense));
        };

        let authors: Option<Authors> = if let Some(cli_authors) = cli.authors.clone() {
            // user passed authors on the command line
            Some(cli_authors)
        } else if let Some(idx) = index {
            // fall back to config’s optional authors
            cfg.licenses.get(idx).and_then(|lic| lic.authors.clone())
        } else {
            // no CLI, no config, no author.
            None
        };

        let date = if let Some(cli_date) = cli.date {
            cli_date
        } else if let Some(idx) = index {
            cfg.licenses
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

        let ignore_git_ignore = cli
            .ignore_git_ignore
            .or_else(|| cfg.ignore_git_ignore)
            .unwrap_or_else(|| false);

        Ok(GenSettings {
            license,
            ignore_git_ignore,
            authors,
            date,
            multiple,
        })
    }
}

/// Handles the `gen` command logic.
pub fn handle_gen(settings: &GenSettings) -> Result<(), FileProcessingError> {
    debug!("Starting handle_gen with args: {:?}", settings);

    // --- Parameter Resolution (CLI vs. Config - Placeholder) ---
    // TODO: Load license, author, year from config if not provided in args.
    let license = settings.license;
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

    if multiple && fs::exists(&output_filename)? {
        output_filename.set_file_name(license.spdx_id().to_string() + "_" + "LICENSE");
    }
    fs::write(&output_filename, rendered_license)?;
    info!("License file written to '{}'", output_filename.display());
    // ---

    Ok(())
}
