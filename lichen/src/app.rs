//! # Application Logic
//!
//! Defines the main application struct `LichenApp` and its core execution logic.

use crate::cli::Commands;
use crate::commands::{apply, generate, init}; // Import handlers
use crate::config::Config;
use crate::error::FileProcessingError;
use log::debug;

/// The main application structure for Lichen.
pub struct LichenApp {}

impl LichenApp {
    /// Creates a new instance of the Lichen application.
    ///
    /// Currently initializes with default settings. Future versions might load config here.
    pub fn new() -> Self {
        // TODO: Load configuration here
        debug!("LichenApp instance created.");
        LichenApp {
            // Initialize fields if any
        }
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
    pub async fn run(&self, command: Commands) -> Result<(), FileProcessingError> {
        debug!("Dispatching command: {:?}", command);
        let cfg = Config::load(".lichenrc")?;
        match command {
            Commands::Gen(args) => {
                // If there are no licenses in your config, CLI only, just one!
                if cfg.licenses.is_empty() {
                    let settings = generate::GenSettings::new(args, cfg, None)?;
                    generate::handle_gen(&settings)?;
                } else {
                    // Fallback loop through each license by index
                    for (idx, _license) in cfg.licenses.iter().enumerate() {
                        let settings = generate::GenSettings::new(args, cfg, Some(idx))?;
                        generate::handle_gen(args, &settings)?;
                    }
                }

                Ok(())
            }
            Commands::Apply(args) => {
                // Call the async handler function from the apply module
                apply::handle_apply(args).await
            }
            Commands::Init(args) => {
                // Call the handler function from the init module
                init::handle_init(args)
            }
        }
    }
}

// Optional: Implement Default if needed
impl Default for LichenApp {
    fn default() -> Self {
        Self::new()
    }
}
