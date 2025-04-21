//! # Lichen
//!
//! A license management cli tool.

// Declare modules
mod app;
mod cli;
mod commands;
mod config;
mod error;
mod license; // Ensure this module exists
mod models;
mod paths;
mod utils;

// Use necessary items from modules
use app::LichenApp;
use cli::Cli;
use error::LichenError; // Use the specific error type

// Std library imports
use std::error::Error;
use std::process::ExitCode;

// External crate imports
use clap::Parser;
use log::{debug, error, info}; // Keep top-level log imports

// Main application logic
#[tokio::main]
async fn main() -> ExitCode {
    // 1. Parse CLI arguments
    // Cli::parse handles errors and exits automatically
    let cli = Cli::parse();

    // 2. Initialize logging
    // Uses clap_verbosity_flag to set level based on -v, -vv, etc.
    // Also respects RUST_LOG environment variable.
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Lichen starting...");
    debug!("CLI arguments parsed: {:?}", cli);
    debug!("Effective log level: {}", cli.verbose.log_level_filter());

    // --- Configuration Loading Placeholder ---
    // TODO: Implement robust configuration loading (e.g., from .lichenrc, XDG dirs)
    // TODO: Merge CLI arguments with configuration (CLI takes precedence).
    // This logic could potentially live within LichenApp::new() or be passed to it.
    debug!("Configuration loading step (currently placeholder).");
    // ---

    // 3. Create the application instance
    let lichen_app = LichenApp::new();

    // 4. Run the dispatched command
    let result = lichen_app.run(cli.command).await; // Pass the command enum

    // 5. Handle command result and exit
    match result {
        Ok(_) => {
            info!("Command executed successfully.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            // Log the specific error using the Display impl from error.rs
            error!("Command failure, because of... a {}", e);
            ExitCode::FAILURE
        }
    }
}
