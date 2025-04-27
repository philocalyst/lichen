//! # Configuration support
//!
//! Manages the loading of options from a TOML config input

use crate::error::LichenError;
use crate::license::License;
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

/// Author struct
#[derive(Debug, Deserialize, Clone)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct Authors(pub Vec<Author>);

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
