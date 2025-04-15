use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;
use std::process::ExitCode;

/// A rust-license management cli tool and library
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a license file
    Gen(GenArgs),
    /// Apply license headers to files
    Apply(ApplyArgs),
    /// Initialize configuration file
    Init(InitArgs),
}

#[derive(Args, Debug)]
struct GenArgs {
    #[arg()]
    license: Option<String>, // Made optional here to allow config fallback later

    /// Author names (comma-separated).
    #[arg(short, long, value_delimiter = ',')]
    author: Option<Vec<String>>,

    /// Year for the license (defaults to current year if blank).
    #[arg(short, long)]
    year: Option<u16>,
}

#[derive(Args, Debug)]
struct ApplyArgs {
    /// Name of the license header to apply (e.g., MIT, Apache-2.0).
    /// Required, but can be read from config if not provided via CLI.
    #[arg()] // Positional argument
    license: Option<String>, // Made optional here to allow config fallback later

    /// Apply headers in-place, modifying original files.
    /// If not set, creates copies in a 'licensed/' directory (or similar).
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    in_place: bool,

    /// Comma-separated list of regex patterns for files/directories to exclude.
    #[arg(short, long, value_delimiter = ',')]
    exclude: Option<Vec<String>>,

    #[arg(default_value = ".")]
    target: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct InitArgs {
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Initialize logging based on verbosity flags and LOG_LEVEL env var.
    // env_logger respects RUST_LOG/LOG_LEVEL by default.
    // So we just need to synthesize clap's verbosity flags.
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    log::info!("Starting license tool"); // Example log

    // ▰▰▰ CONFIGURATION LOADING ▰▰▰ //
    // TODO: XDG Config Loading (using dirs_next::config_dir())
    // TODO: Merge CLI arguments with config (CLI takes precedence).

    let result = match cli.command {
        Commands::Gen(args) => run_gen(args),
        Commands::Apply(args) => run_apply(args),
        Commands::Init(args) => run_init(args),
    };

    match result {
        Ok(_) => {
            log::info!("Command executed successfully.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            log::error!("Error executing command: {}", e);
            // Return a specific error code if desired, otherwise use a generic one.
            ExitCode::FAILURE
        }
    }
}

// ▰▰▰ SUBCOMMAND LOGIC ▰▰▰ //

fn run_gen(args: GenArgs) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Running Gen command with args: {:?}", args);

    // TODO: Config parsing
    let license_name = args
        .license
        .ok_or("License name is required via CLI or config")?;
    let authors = args.author; //.or_else(|| /* get from config */);
    let year = args
        .year
        .unwrap_or_else(|| chrono::Utc::now().format("%Y").to_string().parse().unwrap());

    log::info!("Generating license: {}", license_name);
    if let Some(ref authors_vec) = authors {
        log::info!("Authors: {}", authors_vec.join(", "));
    }
    log::info!("Year: {}", year);

    // TODO: Implement actual license generation logic.
    // - Fetch license template based on `license_name`.
    // - Fill in placeholders (author, year).
    // - Write to a LICENSE file (or stdout if passed through pipe).

    println!(
        "Placeholder: Would generate license '{}' for year {} by {:?}",
        license_name,
        year,
        authors.unwrap_or_default()
    );

    Ok(())
}

fn run_apply(args: ApplyArgs) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Running Apply command with args: {:?}", args);

    // TODO: Add config loading
    let license_name = args
        .license
        .ok_or("License name is required via CLI or config")?;
    let exclude_patterns = args.exclude;

    log::info!("Applying license header: {}", license_name);
    log::info!("In-place modification: {}", args.in_place);
    if let Some(ref patterns) = exclude_patterns {
        log::info!("Excluding patterns: {:?}", patterns);
        // TODO: Add regex
    }

    // TODO: Implement actual license application logic.
    // - Find relevant files (e.g., walk the current directory).
    // - Filter files based on `exclude_patterns`.
    // - Read license header template based on `license_name`.
    // - For each file:
    //   - Check if header already exists.
    //   - If `in_place` is true, prepend header to the file. Using chars
    //   - If `in_place` is false, copy file to a 'licensed/' dir and prepend header there.

    println!(
        "Placeholder: Would apply license '{}' header. In-place: {}. Exclude: {:?}",
        license_name,
        args.in_place,
        exclude_patterns.unwrap_or_default()
    );

    Ok(())
}

fn run_init(args: InitArgs) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Running Init command with args: {:?}", args);

    let project_root = args
        .path
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));
    log::info!("Initializing configuration in: {:?}", project_root);

    // TODO: Implement config file generation.
    // - Determine config file path (e.g., project_root/.licenserc or XDG path).
    // - Pull default values (maybe embed them or have a default template).
    // - Pull cli options
    // - Write the default config file.

    println!(
        "Placeholder: Would generate default config file at {:?}",
        project_root
    );

    Ok(())
}
