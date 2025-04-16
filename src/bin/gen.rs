use heck::ToUpperCamelCase; // Keep this for non-acronym parts
use metadata_gen::extract_and_prepare_metadata;
use quote::{format_ident, quote};
use std::{env, fs, io::Write, path::PathBuf};
use tokio;

// --- Configuration (same as before) ---
const LICENSE_DIR: &str = "licenses";
// --- End Configuration ---

// Helper function to check if a string slice consists entirely of uppercase ASCII letters
fn is_all_caps_acronym(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_uppercase())
}

async fn is_osi(license_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    use metadata_gen::utils::async_extract_metadata_from_file;

    // Extract metadata from the provided license_path instead of the hardcoded path
    let (metadata, _, _) =
        async_extract_metadata_from_file(&license_path.to_string_lossy().to_string()).await?;

    // Check if osiApproved is in the metadata and return its value
    if let Some(osi_status) = metadata.get("osiApproved") {
        if osi_status != "unknown" {
            // Check for edge case where there is no decision.
            if osi_status
                .trim()
                .parse::<bool>()
                .expect("Should include osi approved key in meta")
                == true
            // Convert the value to bool
            {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false) // If marked unknown, safe to say it's not OSI in a way that you could trust.
        }
    } else {
        // If osiApproved key is nonexistent, assume false.
        Ok(false)
    }
}

fn main() -> std::io::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let generated_file_path = out_dir.join("generated_licenses.rs");
    let license_dir_path = PathBuf::from(LICENSE_DIR);

    let mut variants = Vec::new();
    let mut license_details = Vec::new(); // (variant_ident, filename_str, is_osi_bool)

    if license_dir_path.is_dir() {
        for entry in fs::read_dir(&license_dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let (Some(file_stem), Some(file_name_osstr)) =
                    (path.file_stem(), path.with_extension("").file_name())
                {
                    let file_stem_str = file_stem.to_string_lossy();
                    let file_name_str = file_name_osstr.to_string_lossy().to_string(); // Keep original filename

                    // --- Step 1: Number and Dot Replacements ---
                    let mut stem_with_replacements = String::new();
                    for c in file_stem_str.chars() {
                        match c {
                            '0' => stem_with_replacements.push_str("Zero"),
                            '1' => stem_with_replacements.push_str("One"),
                            '2' => stem_with_replacements.push_str("Two"),
                            '3' => stem_with_replacements.push_str("Three"),
                            '4' => stem_with_replacements.push_str("Four"),
                            '5' => stem_with_replacements.push_str("Five"),
                            '6' => stem_with_replacements.push_str("Six"),
                            '7' => stem_with_replacements.push_str("Seven"),
                            '8' => stem_with_replacements.push_str("Eight"),
                            '9' => stem_with_replacements.push_str("Nine"),
                            '+' => stem_with_replacements.push_str("Plus"),
                            '.' => stem_with_replacements.push_str("Dot"),
                            _ => stem_with_replacements.push(c),
                        }
                    }

                    // --- Step 2: Split into parts based on common delimiters ---
                    // We split by non-alphanumeric characters to separate words/acronyms
                    let parts: Vec<&str> = stem_with_replacements
                        .split(|c: char| !c.is_ascii_alphanumeric())
                        .filter(|s| !s.is_empty()) // Remove empty strings resulting from splits
                        .collect();

                    // --- Step 3 & 4: Process Parts (Acronyms/CamelCase) & Join ---
                    let mut final_ident_string = String::new();
                    for part in parts {
                        if is_all_caps_acronym(part) {
                            // Append acronym part as is (e.g., "GPL", "MIT")
                            final_ident_string.push_str(part);
                        } else {
                            // Convert non-acronym parts to UpperCamelCase (e.g., "only" -> "Only")
                            final_ident_string.push_str(&part.to_upper_camel_case());
                        }
                    }

                    // Handle cases where the split results in an empty identifier (e.g., filename was just "-.--")
                    if final_ident_string.is_empty() {
                        eprintln!(
                            "cargo:warning=Could not generate valid identifier for filename: {}",
                            file_name_str
                        );
                        // Optionally create a placeholder identifier or skip this file
                        // For now, we'll create a potentially invalid identifier to highlight the issue
                        final_ident_string.push_str("InvalidLicenseName");
                    }

                    // --- Step 5: Format the final identifier ---
                    let variant_ident = format_ident!("{}", final_ident_string);

                    // --- End Modification ---
                    let is_osi = tokio::runtime::Runtime::new().unwrap().block_on(async {
                        match is_osi(&path).await {
                            Ok(val) => val,
                            Err(_) => false, // Handle any errors by defaulting to false
                        }
                    });
                    variants.push(variant_ident.clone()); // Collect idents for enum definition
                    // Store the ORIGINAL filename string along with the generated variant ident and OSI status
                    license_details.push((variant_ident, file_name_str, is_osi));
                }
            }
        }
    } else {
        eprintln!(
            "cargo:warning=License directory '{}' not found or not a directory. Generating empty License enum.",
            LICENSE_DIR
        );
    }

    // --- Generate Code using quote! (No changes needed here from previous version) ---

    let variants_with_attrs = license_details.iter().map(|(variant, filename, _)| {
        quote! {
            #[value(name = #filename)] // Original SPDX id
            #variant
        }
    });

    let name_match_arms = license_details.iter().map(|(variant, filename, _)| {
        quote! { Self::#variant => #filename }
    });
    let is_osi_match_arms = license_details.iter().map(|(variant, _, is_osi)| {
        quote! { Self::#variant => #is_osi }
    });

    // Retrive just the variant idents for the iter() method
    let variant_idents = license_details.iter().map(|(variant, _, _)| variant);

    let generated_code = quote! {
        #![allow(clippy::all)]

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[allow(non_camel_case_types)] // Still useful for edge cases
        pub enum License {
            #( #variants ),* }

        impl License {
            /// Returns the original filename of the license (e.g., "Apache-2.0.md").
            pub fn name(&self) -> &'static str {
                match self {
                    #( #name_match_arms ),* }
            }

            /// Returns true if the license is OSI approved (based on the build-time list).
            pub fn is_osi_approved(&self) -> bool {
                match self {
                    #( #is_osi_match_arms ),* }
            }

            /// Returns an iterator over all available license variants.
            pub fn iter() -> impl Iterator<Item = Self> {
                 [ #( Self::#variant_idents ),* ].iter().copied()
            }
        }

        // Optional: FromStr, Display implementations...
    };

    fs::write(&generated_file_path, generated_code.to_string())?;

    // --- Tell Cargo when to rerun the script (same as before) ---
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", LICENSE_DIR);
    if license_dir_path.is_dir() {
        for entry in fs::read_dir(&license_dir_path)? {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    println!("cargo:rerun-if-changed={}", entry.path().display());
                }
            }
        }
    }

    Ok(())
}
