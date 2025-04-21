use crate::error::LichenError;
use crate::license::License;
use jiff::civil::Date;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Top‑level configuration.
#[derive(Debug, Deserialize, Default)]
pub struct Config {
    /// When applying headers, which kind of comment token the user *wants*
    /// Completely possible line or block doesn't exist, in which case it falls back to the other.
    #[serde(default)]
    pub prefer_block: Option<bool>,

    /// Apply headers in-place, modifying the original files.
    /// Caution: This modifies files directly. Ensure backups or version control.
    #[serde(default)]
    pub in_place: Option<bool>,

    // By default conflicts from multiple licenses will warn and replace instead of merging
    #[serde(default)]
    pub multiple: Option<bool>,

    // Global exclude list
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_regex", default)]
    pub exclude: Option<Vec<Regex>>,

    // By default conflicts from multiple licenses will error instead of merging
    #[serde(default)]
    pub ignore_git_ignore: Option<bool>,

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
            Ok(cfg) => Ok(cfg),
            Err(LichenError::IoError(ref io_err))
                if io_err.kind() == std::io::ErrorKind::NotFound =>
            {
                // no file → empty‐config
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
        let wrapper_char_left = "[";
        let wrapper_char_right = "]";

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
        // Join each author’s Display with commas
        let joined = self
            .0
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", joined)
    }
}

// Helper to deserialize a single Regex from a TOML string.
fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Regex::new(&s).map_err(serde::de::Error::custom)
}

// Helper to deserialize a Vec<Regex> from a TOML string array.
fn deserialize_vec_regex<'de, D>(deserializer: D) -> Result<Vec<Regex>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<String> = Vec::deserialize(deserializer)?;
    raw.into_iter()
        .map(|s| Regex::new(&s).map_err(serde::de::Error::custom))
        .collect()
}
