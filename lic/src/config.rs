//! # Configuration support
//!
//! Manages the loading of options from a TOML config input

use crate::error::LichenError;
use crate::models::License;
use crate::models::{Author, Authors};
use jiff::civil::Date;
use log::{debug, warn};
use regex::Regex;
use serde::Deserialize;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Complete configuration. Holds fields for a the sum of both Apply and Gen args
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// When applying headers, which kind of comment token the user *wants*
    /// Completely possible line or block doesn't exist, in which case it falls back to the other.
    #[serde(default)]
    pub prefer_block: Option<bool>,

    // By default conflicts from multiple licenses will warn and replace instead of merging
    #[serde(default)]
    pub multiple: Option<bool>,

    // Global exclude list
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_regex", default)]
    pub exclude: Option<Vec<Regex>>,

    // By default conflicts from multiple licenses will error instead of merging
    #[serde(default)]
    pub all: Option<bool>,

    /// Per‑license configuration blocks.
    #[serde(rename = "license", default)]
    pub licenses: Option<Vec<LicenseConfig>>,
}

/// Try to load and parse the config file.
/// Converts I/O or parse errors into your `FileProcessingError`.
impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, LichenError> {
        let s = fs::read_to_string(path.as_ref()).map_err(LichenError::from)?;
        toml::from_str(&s).map_err(|e| LichenError::Msg(format!("config parse error: {}", e)))
    }

    /// Like `load`, but if the file was *not found*, you get `Config::default()`.
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self, LichenError> {
        match Self::load(&path) {
            Ok(cfg) => {
                debug!("Running with config");
                Ok(cfg)
            }
            Err(LichenError::IoError(ref io_err))
                if io_err.kind() == std::io::ErrorKind::NotFound =>
            {
                // no file → empty‐config
                warn!("No config found, falling back on CLI and defaults");
                Ok(Config::default())
            }
            Err(other) => Err(other),
        }
    }
}

/// Per‑license settings.
#[derive(Debug, Deserialize)]
pub struct LicenseConfig {
    /// Regex for matching file paths to apply this license.
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_regex", default)]
    pub exclude: Option<Regex>,

    /// File‑path patterns, files or directories..
    #[serde(default)]
    pub targets: Option<Vec<PathBuf>>,

    // Provided date
    #[serde(default)]
    pub date: Option<Date>,

    /// SPDX identifier.
    pub id: License,

    /// List of named authors.
    #[serde(default)]
    pub authors: Option<Authors>,
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Open to this being changed, just what made sense at the time.
        let wrapper_char_left = "[";
        let wrapper_char_right = "]";

        // Ex output: John Pork [johnpork@pig.com]
        if let Some(email) = &self.email {
            write!(
                f,
                "{name} {left}{email}{right}",
                name = self.name,
                left = wrapper_char_left,
                email = email,
                right = wrapper_char_right
            )
        } else {
            write!(f, "{name}", name = self.name)
        }
    }
}

impl fmt::Display for Authors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Simple and easy, just get the string representation of each author, and join with comma.
        let joined = self
            .0
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", joined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn author_display_with_email() {
        let a = Author {
            name: "Alice".into(),
            email: Some("a@e.com".into()),
        };
        let s = format!("{}", a);
        assert_eq!(s, "Alice [a@e.com]");
    }

    #[test]
    fn author_display_no_email() {
        let a = Author {
            name: "Bob".into(),
            email: None,
        };
        let s = format!("{}", a);
        assert_eq!(s, "Bob");
    }

    #[test]
    fn authors_display_multiple() {
        let authors = Authors(vec![
            Author {
                name: "X".into(),
                email: None,
            },
            Author {
                name: "Y".into(),
                email: Some("y@z".into()),
            },
        ]);
        let s = format!("{}", authors);
        assert_eq!(s, "X, Y [y@z]");
    }
}

#[cfg(test)]
mod tests_load {
    // Separate module to avoid conflicts with existing tests mod
    use super::*;
    use crate::models::License;
    use std::fs;
    use tempfile::NamedTempFile; // Import License

    #[test]
    fn config_load_valid_toml() {
        let content = r#"
# Configuration for Lichen, a tool for managing licenses
# This file allows you to specify global settings and per-license configurations.

# ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰ #
# Global Configuration #
# ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰ #

# prefer_block = true
# multiple = true

exclude = [
  "\\.gitignore",
  ".*lock",
  "\\.git/.*",
  "\\.licensure\\.yml",
  "README.*",
  "LICENSE.*",
  ".*\\.(md|rst|txt)",
  "Cargo.toml",
  ".*\\.github/.*",
]

# all = true

# ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰ #
# Per-License Configuration #
# ▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰ #

[[license]]
# exclude = "some/pattern/to/exclude"
targets = ["."]
id = "MIT"
# date = "2023-10-27"
authors = [
  { name = "Core Contributor", email = "core@example.com" },
  { name = "Another Contributor" }, # Email is optional
]

# [[license]]
# targets = ["src/cli/"]
# id = "Apache-2.0"
# authors = [
#   { name = "CLI Developer" }
# ]

# [[license]]
# targets = ["examples/"]
# id = "GPL-3.0-or-later"
# date = "2024-01-01"
"#;
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), content).unwrap();

        let config = Config::load(file.path()).unwrap();

        // --- Global Assertions ---
        assert_eq!(config.prefer_block, None); // Commented out
        assert_eq!(config.multiple, None); // Commented out
        assert_eq!(config.all, None); // Commented out

        assert!(config.exclude.is_some());
        let excludes = config.exclude.as_ref().unwrap();
        assert_eq!(excludes.len(), 9); // Updated count

        // --- License Assertions ---
        assert!(config.licenses.is_some());
        let licenses = config.licenses.unwrap();
        assert_eq!(licenses.len(), 1); // Only one license block is active

        // Check the first (and only) license
        let lic1 = &licenses[0];
        assert_eq!(lic1.id.spdx_id(), "MIT"); // Check the raw string ID from TOML

        assert_eq!(
            lic1.targets,
            Some(vec![PathBuf::from(".")]) // Updated target
        );

        assert!(lic1.authors.is_some());
        // lic1.authors is Option<Authors>
        // lic1.authors.as_ref() is Option<&Authors>
        // lic1.authors.as_ref().unwrap() is &Authors
        // lic1.authors.as_ref().unwrap().0 is Vec<Author>
        let authors_vec = &lic1.authors.as_ref().unwrap().0; // Access the inner Vec<&Author> using .0
        assert_eq!(authors_vec.len(), 2); // Two authors specified

        // Check first author
        assert_eq!(authors_vec[0].name, "Core Contributor"); // Now index into the Vec
        assert_eq!(authors_vec[0].email, Some("core@example.com".to_string()));

        // Check second author
        assert_eq!(authors_vec[1].name, "Another Contributor");
        assert_eq!(authors_vec[1].email, None); // Email is optional and not provided

        assert_eq!(lic1.date, None); // Commented out in the license block
    }

    #[test]
    fn config_load_minimal_toml() {
        let content = r#"
# Only specify one license ID
[[license]]
id = "Unlicense"
"#;
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), content).unwrap();
        let config = Config::load(file.path()).unwrap();

        assert!(config.prefer_block.is_none()); // Defaults to None
        assert!(config.multiple.is_none());
        assert!(config.exclude.is_none());
        assert!(config.all.is_none());

        assert!(config.licenses.is_some());
        let licenses = config.licenses.unwrap();
        assert_eq!(licenses.len(), 1);
        assert_eq!(licenses[0].id, License::Unlicense);
        assert!(licenses[0].targets.is_none());
        assert!(licenses[0].authors.is_none());
        assert!(licenses[0].exclude.is_none());
        assert!(licenses[0].date.is_none());
    }

    #[test]
    fn config_load_invalid_toml_returns_err() {
        let content = r#"
prefer_block = true
multiple = "not a boolean" # Invalid type
"#;
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), content).unwrap();

        let result = Config::load(file.path());
        assert!(result.is_err());
        assert!(matches!(result, Err(LichenError::Msg(_))));
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("config parse error")
        );
    }

    #[test]
    fn config_load_or_default_file_not_found_returns_default() {
        let non_existent_path = PathBuf::from("this_file_definitely_does_not_exist.toml");
        let result = Config::load_or_default(&non_existent_path);

        assert!(result.is_ok());
        let config = result.unwrap();
        // Check if it's the default config
        assert!(config.prefer_block.is_none());
        assert!(config.multiple.is_none());
        assert!(config.exclude.is_none());
        assert!(config.all.is_none());
        assert!(config.licenses.is_none());
    }

    #[test]
    fn config_load_or_default_loads_existing_file() {
        let content = r#"prefer_block = true"#;
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), content).unwrap();

        let result = Config::load_or_default(file.path());
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.prefer_block, Some(true));
    }

    #[test]
    fn config_load_or_default_invalid_toml_returns_err() {
        let content = r#"invalid toml content"#;
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), content).unwrap();

        let result = Config::load_or_default(file.path());
        assert!(result.is_err());
        assert!(matches!(result, Err(LichenError::Msg(_))));
    }
}
