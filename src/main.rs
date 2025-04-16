use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use directories::{self, BaseDirs, ProjectDirs};
use regex;
use serde_json;
use std::error::Error;
use std::fs::{self, File, read_to_string};
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::process::ExitCode;
use std::str::FromStr;
use tempfile::NamedTempFile;
mod license;
use license::License;
use walkdir;

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
    license: Option<License>, // Made optional here to allow config fallback later

    /// Author names (comma-separated).
    #[arg(short, long, value_delimiter = ',')]
    author: Option<Vec<String>>,

    // Whether or not to use a markdown license, defaults to true
    #[arg(long, default_value_t = true)]
    markdown: bool,

    /// Year for the license (defaults to current year if blank).
    #[arg(short, long)]
    year: Option<u16>,
}

#[derive(Args, Debug)]
struct ApplyArgs {
    /// Name of the license header to apply (e.g., MIT, Apache-2.0).
    /// Required, but can be read from config if not provided via CLI.
    #[arg()] // Positional argument
    license: Option<License>, // Made optional here to allow config fallback later

    /// Apply headers in-place, modifying original files.
    /// If not set, creates copies in a 'licensed/' directory (or similar).
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    in_place: bool,

    /// Comma-separated list of regex patterns for files/directories to exclude.
    #[arg(short, long, value_delimiter = ',')]
    exclude: Option<String>,

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
    let license = args
        .license
        .ok_or("License name is required via CLI or config")?;
    let authors = args.author; //.or_else(|| /* get from config */);
    let year = args
        .year
        .unwrap_or_else(|| chrono::Utc::now().format("%Y").to_string().parse().unwrap());

    let extention = if args.markdown { "md" } else { "txt" };

    log::info!("Generating license: {}", license);
    if let Some(ref authors_vec) = authors {
        log::info!("Authors: {}", authors_vec.join(", ")); // Fill author field
    }
    log::info!("Year: {}", year);

    // Search through the licenses directory for the matching spdx ID
    let resources_directory =
        if let Some(proj_dirs) = ProjectDirs::from("com", "philocalyst", "lichen") {
            proj_dirs
            // Lin: /home/alice/.config/barapp
            // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
            // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        } else {
            panic!("Could not determine project directory");
        };

    let resources_directory = resources_directory.data_dir();
    let license_path = resources_directory
        .join("license")
        .join(license.spdx_id())
        .with_extension(extention);

    let mut output_file =
        PathBuf::from_str("LICENSE").expect("No IOCreation happens here, so impossible to fail");

    if extention == "md" {
        output_file.set_extension("md");
    } // Without the file extention most programs just read as text...

    fs::copy(license_path, output_file)?;

    // TODO: Implement actual license generation logic.
    // - Fetch license template based on `license_name`.
    // - Fill in placeholders (author, year).
    // - Write to a LICENSE file (or stdout if passed through pipe).

    Ok(())
}

fn generate_blacklist(
    target: &PathBuf,
    reg_pattern: regex::Regex,
) -> std::io::Result<Vec<PathBuf>> {
    let mut blacklist: Vec<PathBuf> = Vec::new();

    for entry in walkdir::WalkDir::new(&target)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path().to_owned();
        if reg_pattern.is_match(&path.to_string_lossy()) {
            blacklist.push(path);
        }
    }

    Ok(blacklist)
}

fn get_valid_files(target: &PathBuf, blacklist: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut response = Vec::new();
    let walker = walkdir::WalkDir::new(target).into_iter();
    for entry in walker.filter_entry(|e| !blacklist.iter().any(|path| path == e.path())) {
        let entry = entry.unwrap().into_path();
        response.push(entry);
    }
    response
}

fn run_apply(args: ApplyArgs) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Running Apply command with args: {:?}", args);

    // TODO: Add config loading
    let license_name = args
        .license
        .ok_or("License name is required via CLI or config")?;
    let exclude_pattern = regex::Regex::new(&args.exclude.unwrap());
    let target = args.target.unwrap();

    log::info!("Applying license header: {}", license_name);
    log::info!("In-place modification: {}", args.in_place);
    log::info!("Excluding pattern: {:?}", exclude_pattern);

    let search_dir = "licenses";
    let found_path: Option<PathBuf> = seek_license(search_dir, &license_name); // Variable to store the path if found

    let license_path = if let Some(path) = found_path {
        path
    } else {
        eprintln!(
            "License file '{}' not found in directory '{}'.",
            license_name, search_dir
        );
        process::exit(1);
    };

    let license_path: PathBuf = get_license_path(&license, "txt");

    let blacklist = generate_blacklist(&target, exclude_pattern?);

    let license_content = fs::read_to_string(license_path).unwrap();

    let working_files = get_valid_files(&target, &blacklist.unwrap());

    prepend_files(&license_content, working_files);

    // TODO: Implement actual license application logic.
    // - Find relevant files (e.g., walk the current directory).
    // - Filter files based on `exclude_patterns`.
    // - Read license header template based on `license_name`.
    // - For each file:
    //   - Check if header already exists.
    //   - If `in_place` is true, prepend header to the file. Using chars
    //   - If `in_place` is false, copy file to a 'licensed/' dir and prepend header there.

    println!(
        "Placeholder: Would apply license '{}' header. In-place: {}. Exclude: ",
        license.spdx_id(),
        args.in_place
    );

    Ok(())
}

fn prepend_files(license: &String, paths: Vec<PathBuf>) {
    for path in paths {
        let comment_char = if let Some(ext) = path.extension() {
            get_comment_char(ext.to_str().unwrap_or(""))
        } else {
            get_comment_char("")
        };
        let license = apply_comments(license, comment_char.unwrap());
        prepend_file(&license, path);
    }
}

fn get_comment_char(extension: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resources_directory =
        if let Some(proj_dirs) = ProjectDirs::from("com", "philocalyst", "lichen") {
            proj_dirs
            // Lin: /home/alice/.config/barapp
            // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
            // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        } else {
            panic!("Could not determine project directory");
        };

    let resources_directory = resources_directory.data_dir();
    let comment_tokens_path = resources_directory.join("comment-tokens.json");

    if !resources_directory.try_exists()? {
        fs::create_dir(resources_directory)?;
        if !comment_tokens_path.try_exists()? {
            fs::write(&comment_tokens_path, "")?;
        }
    }
    let file = File::open(comment_tokens_path)?;
    let reader = BufReader::new(file);
    // Open and read the JSON file
    let data: serde_json::Value = serde_json::from_reader(reader)?;

    // Check if data is an object (expected)
    let languages_map = data
        .as_object()
        .ok_or_else(|| "JSON data is not a top-level object".to_string())?;

    // Iterate through the languages
    for (_, language_details) in languages_map {
        // Check if this language has the target extension
        if let Some(file_types) = language_details.get("file_types") {
            if let Some(file_types_array) = file_types.as_array() {
                // Check if the provided extention is in the file_types array for the language
                let has_extension = file_types_array
                    .iter()
                    .any(|ext| ext.as_str().map_or(false, |s| s == extension));

                if has_extension {
                    // Get comment toeken for the language
                    if let Some(token_value) = language_details.get("comment_token") {
                        if let Some(token_str) = token_value.as_str() {
                            return Ok(token_str.to_string());
                        }
                    }

                    // If no comment token but extension matches return an error
                    return Ok("#".to_string());
                }
            }
        }
    }

    // If no matching language is found
    Ok("#".to_string())
}

fn apply_comments(license: &String, com_char: String) -> String {
    let mut response = String::new();
    for line in license.split('\n') {
        response.push_str(&format!("{}{}\n", com_char, line));
    }
    response
}

fn strip_metadata(data: Vec<char>) -> String {
    let mut result = String::new();

    // If the string is too short to contain metadata, safe to return early.
    if data.len() < 6 {
        // Minimum: "---\n---"
        return data.iter().collect();
    }

    // If the document starts with "---" it has metadata.
    if data.len() >= 3 && data[0] == '-' && data[1] == '-' && data[2] == '-' {
        // Look for the closing "---"
        let mut i = 3;
        let mut line_start = true;
        let mut found_closing = false;

        while i < data.len() - 2 {
            if line_start && data[i] == '-' && data[i + 1] == '-' && data[i + 2] == '-' {
                // Found closing "---", skip past it
                i += 3;
                found_closing = true;
                break;
            }

            // Track if we're at the start of a new line
            line_start = data[i] == '\n';
            i += 1;
        }

        // If we found both opening and closing markers, append everything after
        if found_closing {
            // Skip any whitespace after the closing ---
            while i < data.len()
                && (data[i] == '\n' || data[i] == '\r' || data[i] == ' ' || data[i] == '\t')
            {
                i += 1;
            }

            // Append the rest of the content
            result.extend(data[i..].iter());
        } else {
            // If no closing marker was found, return the original string
            return data.iter().collect();
        }
    } else {
        // No metadata block detected, return the original string
        return data.iter().collect();
    }

    result
}

fn prepend_file<P: AsRef<Path>>(license: &String, file_path: P) -> io::Result<()> {
    // Create a temporary file in the same directory for better cross-device moves
    let dir = file_path
        .as_ref()
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let mut tmp = NamedTempFile::new_in(dir)?;

    // Write the data to prepend
    tmp.write_all(license.as_bytes())?;

    // Open source file, read its contents, and write to the temp file
    let mut src_content = Vec::new();
    File::open(&file_path)?.read_to_end(&mut src_content)?;
    tmp.write_all(&src_content)?;

    // Atomically replace the original file with the temporary file
    tmp.persist(file_path)?;

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
