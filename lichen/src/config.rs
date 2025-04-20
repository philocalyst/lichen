use crate::error::FileProcessingError;
use crate::license::License;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Top‑level configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Running in place by default
    #[serde(default)]
    pub change_in_place: bool,

    // By default conflicts from multiple licenses will error instead of merging
    #[serde(default)]
    pub error_on_conflict: bool,

    /// File‑path exclusion patterns.
    #[serde(default, deserialize_with = "deserialize_vec_regex")]
    pub excludes: Vec<Regex>,

    /// Per‑license configuration blocks.
    #[serde(rename = "license")]
    pub licenses: Vec<LicenseConfig>,
}

/// Per‑license settings.
#[derive(Debug, Deserialize)]
pub struct LicenseConfig {
    /// Regex for matching file paths to apply this license.
    #[serde(deserialize_with = "deserialize_regex")]
    pub files: Option<Regex>,

    /// SPDX identifier.
    pub id: License,

    // By default the program will attempt to generate licenses instead of headerizing
    #[serde(default)]
    pub headerize: bool,

    /// List of named authors.
    #[serde(default)]
    pub authors: Vec<Author>,
}

/// Author struct
#[derive(Debug, Deserialize)]
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
