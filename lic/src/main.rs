//! # Main
//!
//! Main is where the high-level execution flow is handled

// Application modules
mod app;
mod commands;
mod config;
mod error;
mod license;
mod models;
mod utils;

// Core imports
use crate::models::Cli;
use app::LichenApp;

// STD
use std::process::ExitCode;

// External crate imports
use clap::Parser;
use log::{debug, error, trace};

// Main application logic
#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    // Initalize the logging
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    trace!("Lichen starting...");
    debug!("CLI arguments parsed: {:?}", cli);
    debug!("Effective log level: {}", cli.verbose.log_level_filter());

    debug!("Configuration loading step (currently placeholder).");

    // Create the instance
    let lichen_app = LichenApp::new();

    // See if there's a config to worry over
    let config_path = cli.config.unwrap_or(".lichen.toml".into());

    // Run the command with the sourced configuration
    let result = lichen_app.run(cli.command, config_path).await;

    // Handle any errors and exit :)
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("Command failure, because of... a {}", e);
            ExitCode::FAILURE
        }
    }
}
