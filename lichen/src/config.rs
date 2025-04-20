use crate::error::FileProcessingError;
use crate::license::License;
use jiff::civil::Date;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Top‑level configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Running in place by default
    #[serde(default)]
    pub change_in_place: Option<bool>,

    // By default conflicts from multiple licenses will error instead of merging
    #[serde(default)]
    pub error_on_conflict: Option<bool>,

    // By default conflicts from multiple licenses will error instead of merging
    #[serde(default)]
    pub ignore_git_ignore: Option<bool>,

    /// Per‑license configuration blocks.
    #[serde(rename = "license")]
    pub licenses: Vec<LicenseConfig>,
}

/// Per‑license settings.
#[derive(Debug, Deserialize)]
pub struct LicenseConfig {
    /// Regex for matching file paths to apply this license.
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_regex")]
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
    pub authors: Option<Vec<Author>>,
}

/// Author struct
#[derive(Debug, Deserialize, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
}

/// Load and parse a TOML config file from the path
impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, FileProcessingError> {
        let s = fs::read_to_string(path).map_err(FileProcessingError::IoError)?;
        toml::from_str(&s).map_err(|e| FileProcessingError::Msg(e.to_string()))
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
