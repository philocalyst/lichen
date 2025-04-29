// build.rs
use clap::{CommandFactory, ValueEnum};
use clap_complete::{Shell, generate_to};
use std::env;
use std::error::Error;

include!("src/models.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let outdir = match env::var_os("OUT_DIR") {
        None => {
            eprintln!(
                "cargo:warning=OUT_DIR environment variable not found. Skipping completion generation."
            );
            return Ok(());
        }
        Some(outdir) => PathBuf::from(outdir), // Convert OsString to PathBuf
    };

    // Use CommandFactory to get the command structure from the Cli struct (defined via include!)
    let mut cmd = Cli::command(); // Cli is now defined via the include!
    let bin_name = env!("CARGO_PKG_NAME"); // Get bin name from Cargo.toml

    eprintln!(
        "Generating completions for '{}' into: {:?}",
        bin_name, outdir
    );

    for shell in Shell::value_variants() {
        let path = generate_to(*shell, &mut cmd, bin_name, &outdir); // Pass outdir by reference
        match path {
            Ok(p) => {
                eprintln!("Generated completion file for {:?}: {:?}", shell, p);
            }
            Err(e) => {
                // Don't fail the build for a completion error, just warn
                eprintln!(
                    "cargo:warning=Failed to generate completion file for {:?}: {}",
                    shell, e
                );
            }
        }
    }

    // Tell cargo to rerun the build script if models.rs changes.
    println!("cargo:rerun-if-changed=src/models.rs");
    // Keep if build.rs itself changes
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
