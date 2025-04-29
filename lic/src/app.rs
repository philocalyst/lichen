//! # Application Logic
//!
//! Defines the main application struct `LichenApp` and its core execution logic.

use std::path::PathBuf;

use crate::commands::{apply, generate, init, unapply}; // Import handlers
use crate::config::Config;
use crate::error::LichenError;
use crate::models::Commands;
use log::debug;

/// The main application structure for Lichen.
pub struct LichenApp {}

impl LichenApp {
    /// Creates a new instance of the Lichen application.
    ///
    /// Currently initializes with default settings.
    pub fn new() -> Self {
        debug!("LichenApp instance created.");
        LichenApp {}
    }

    /// Runs the command specified by the parsed CLI arguments.
    ///
    /// # Arguments
    ///
    /// * `command`: The `Commands` enum variant representing the subcommand to execute.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `FileProcessingError`.
    pub async fn run(&self, command: Commands, config_path: PathBuf) -> Result<(), LichenError> {
        debug!("Dispatching command: {:?}", command);
        let cfg = Config::load_or_default(config_path)?;
        match command {
            Commands::Gen(args) => {
                // If there are no licenses in your configuration, no need for multiple runs, and so you can safely fallback to a single run.
                if cfg.licenses.is_none() {
                    let settings = generate::GenSettings::new(&args, &cfg, None)?;
                    generate::handle_gen(&settings)?;
                } else {
                    // Fallback loop through each and every license by index
                    for (idx, _license) in cfg.licenses.iter().enumerate() {
                        let settings = generate::GenSettings::new(&args, &cfg, Some(idx))?;
                        generate::handle_gen(&settings)?;
                    }
                }

                Ok(()) // No errs
            }
            Commands::Apply(args) => {
                // If there are no licenses in your configuration, no need for multiple runs, and so you can safely fallback to a single run.
                if cfg.licenses.is_none() {
                    let settings = apply::ApplySettings::new(&args, &cfg, None)?;
                    apply::handle_apply(&settings).await?;
                } else {
                    // Fallback loop through each license by index
                    for (idx, _license) in cfg.licenses.iter().enumerate() {
                        let settings = apply::ApplySettings::new(&args, &cfg, Some(idx))?;
                        apply::handle_apply(&settings).await?;
                    }
                }
                Ok(()) // No errs
            }
            Commands::Init(args) => init::handle_init(args), // CLI only
            Commands::Unapply(args) => unapply::handle_unapply(args).await, // CLI only
        }
    }
}

impl Default for LichenApp {
    fn default() -> Self {
        Self::new()
    }
}
