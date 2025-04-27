//! # Initialize Command
//!
//! Logic for the `lichen init` command (currently placeholder).

use crate::cli::InitArgs;
use crate::error::LichenError;
use log::debug;
use std::fs;

/// Handles the `init` command logic (currently a placeholder).
pub fn handle_init(args: InitArgs) -> Result<(), LichenError> {
    debug!("Starting handle_init with args: {:?}", args);

    let project_root = match args.target {
        Some(p) => {
            debug!("Using specified path for initialization: '{}'", p.display());
            p
        }
        None => {
            let cwd = std::env::current_dir()?;
            debug!(
                "No path specified, using current working directory: '{}'",
                cwd.display()
            );
            cwd
        }
    };

    debug!(
        "Initializing default configuration in: '{}'",
        project_root.display()
    );

    fs::write(
        ".lichen.toml",
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/default.toml")),
    )?;

    Ok(())
}
