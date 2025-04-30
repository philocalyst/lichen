//! # Main
//!
//! Main is where the high-level execution flow is handled

// Application modules
mod app;
mod cli;
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
    // |1| Parse CLI arguments
    let cli = Cli::parse();

    // |2| Initialize logging
    // Uses clap_verbosity_flag to set level based on -v, -vv, etc.
    // Also respects RUST_LOG environment variable.
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    trace!("Lichen starting...");
    debug!("CLI arguments parsed: {:?}", cli);
    debug!("Effective log level: {}", cli.verbose.log_level_filter());

    debug!("Configuration loading step (currently placeholder).");

    // |3| Create the application instance
    let lichen_app = LichenApp::new();

    // Find config
    let config_path = cli.config.unwrap_or(".lichen.toml".into());

    // |4| Run the dispatched command
    let result = lichen_app.run(cli.command, config_path).await; // Pass the command enum

    // |5| Handle command results and exit
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("Command failure, because of... a {}", e);
            ExitCode::FAILURE
        }
    }
}
