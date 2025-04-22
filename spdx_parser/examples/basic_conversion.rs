// examples/basic_conversion.rs
use std::path::PathBuf;
use std::process::ExitCode;
// Use the actual name of your library crate defined in Cargo.toml
use env_logger;
use spdx_parser::convert_templates;

fn main() -> ExitCode {
    // Initialize logging *in the application*
    // Set RUST_LOG environment variable (e.g., RUST_LOG=info)
    env_logger::init();

    // --- Configuration ---
    // Replace with your actual input and output paths
    let input_dir = PathBuf::from("./input_templates"); // Example path
    let output_dir = PathBuf::from("./output_markdown"); // Example path
                                                         // ---------------------

    // Create dummy input dir/files for testing if they don't exist
    if !input_dir.exists() {
        if let Err(e) = create_dummy_input(&input_dir) {
            log::error!("Failed to create dummy input: {}", e);
            return ExitCode::FAILURE;
        }
    }

    log::info!(
        "Application: Starting conversion from '{}' to '{}'",
        input_dir.display(),
        output_dir.display()
    );

    // Call the library function
    match convert_templates(&input_dir, &output_dir) {
        Ok(_) => {
            log::info!("Application: Conversion completed successfully.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            log::error!("Application: Conversion failed: {}", e);
            // You might want more specific error handling here
            ExitCode::FAILURE
        }
    }
}

// Helper to create dummy files for quick testing
fn create_dummy_input(input_dir: &PathBuf) -> Result<(), std::io::Error> {
    log::warn!(
        "Creating dummy input directory and files at {}",
        input_dir.display()
    );
    std::fs::create_dir_all(input_dir)?;
    std::fs::write(
        input_dir.join("sample.txt"),
        r#"Hello <<var;name="name";original="World";match=".+">>! <<beginOptional>>Optional text.<<endOptional>>"#,
    )?;
    std::fs::write(
        input_dir.join("sample.html"),
        r#"
         <p>This is a paragraph.</p>
         <div class="replaceable-license-text"><span>Replace Me</span></div>
         <div class="optional-license-text"><p>Optional HTML</p></div>
         <p>Another paragraph with a <var class="replaceable-license-text">variable</var>.</p>
     "#,
    )?;
    Ok(())
}
