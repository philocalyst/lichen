//! # Application Logic
//!
//! Defines the main application struct `LichenApp` and its core execution logic.

use crate::cli::Commands;
use crate::commands::{apply, generate, init}; // Import handlers
use crate::error::FileProcessingError;
use log::debug;

/// The main application structure for Lichen.
pub struct LichenApp {
    // Future fields:
    // config: AppConfig, // Loaded configuration
    // verbosity: log::LevelFilter,
}

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
        match command {
            Commands::Gen(args) => {
                // Call the handler function from the gen module
                generate::handle_gen(args)
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
