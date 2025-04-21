//! # Command Line Interface
//!
//! Defines the CLI structure and argument parsing using Clap.

use crate::config::{Author, Authors};
use crate::license::License;
use clap::{Args, ColorChoice, Error, Parser, Subcommand, builder::styling};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use jiff::civil::Date;
use regex::Regex;
use std::path::PathBuf;

// ▰▰▰ CLI Argument Structs ▰▰▰ //

fn parse_year_to_date(s: &str) -> Result<Date, String> {
    let year: i16 = s
        .parse()
        .map_err(|e| format!("invalid year `{}`: {}", s, e))?;
    Date::new(year, 1, 1).map_err(|e| format!("invalid date: {}", e))
}

pub fn parse_to_author(input: &str) -> Result<Authors, String> {
    // If the whole string is empty or only whitespace, return an empty Vec.
    if input.trim().is_empty() {
        return Err(format!(
            "You need to provide an author in the format NAME:EMAIL"
        ));
    }

    let v = input
        .split(',') // split entries on commas
        .map(str::trim) // trim whitespace around each entry
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            // split exactly Once on ':'
            let mut parts = entry.splitn(2, ':');
            let name = parts
                .next()
                .expect("Always yields at least one element")
                .trim();
            let email = parts
                .next()
                .ok_or_else(|| format!("entry `{}` missing `:` separator", entry))?
                .trim();

            if name.is_empty() {
                Err(format!("entry `{}` has empty name", entry))
            } else if email.is_empty() {
                Err(format!("entry `{}` has empty email", entry))
            } else {
                Ok(Author {
                    name: name.to_string(),
                    email: email.to_string(),
                })
            }
        })
        .collect::<Result<Vec<Author>, String>>()?; // collects into Result<Vec<Author>, String>
    Ok(Authors(v))
}

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

/// A license management cli tool
#[derive(Parser, Debug)]
#[command(author, version, about, styles = STYLES, long_about = None, color = ColorChoice::Auto)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a license file (e.g., LICENSE or LICENSE.md)
    Gen(GenArgs),
    /// Apply license headers to source files
    Apply(ApplyArgs),
    /// Initialize a default configuration file (Not fully implemented)
    Init(InitArgs),
}

#[derive(Args, Debug)]
pub struct GenArgs {
    /// SPDX identifier of the license to generate (e.g., MIT, Apache-2.0).
    /// Can be omitted if specified in configuration.
    #[arg()]
    pub license: Option<License>,

    /// Author names and emails (comma-separated).
    #[arg(short, long, value_parser = parse_to_author)]
    pub authors: Option<Authors>,

    #[arg(long)]
    pub change_in_place: Option<bool>,

    #[arg(long)]
    pub multiple: Option<bool>,

    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,

    /// Generate a Markdown formatted license file (`LICENSE.md`). Defaults to plain text (`LICENSE.txt`).
    // #[arg(long, default_value_t = false)] // Default to false for .txt
    // pub markdown: bool,

    /// Year for the license copyright notice (defaults to the current year).
    #[arg(short, long, value_parser = parse_year_to_date)]
    pub date: Option<Date>,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub ignore_git_ignore: Option<bool>,
}

#[derive(Args, Debug)]
pub struct ApplyArgs {
    /// SPDX identifier of the license header to apply (e.g., MIT, Apache-2.0).
    /// Can be omitted if specified in configuration.
    #[arg()]
    pub license: Option<License>,

    /// Apply headers in-place, modifying the original files.
    /// Caution: This modifies files directly. Ensure backups or version control.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub in_place: Option<bool>,

    #[arg(long)]
    pub multiple: Option<bool>,

    /// When applying headers, which kind of comment token the user *wants*
    /// Completely possible line or block doesn't exist, in which case it falls back to the other.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub prefer_block: Option<bool>,

    /// Year for the license copyright notice (defaults to the current year).
    #[arg(short, long, value_parser = parse_year_to_date)]
    pub date: Option<Date>,

    /// Regex pattern for files/directories to exclude. Applied during directory traversal.
    #[arg(short, long)] // Removed value_delimiter, regex parsing handles it
    pub exclude: Option<Regex>,

    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,

    /// Respect git_ignore option
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub ignore_git_ignore: Option<bool>,

    /// Author names and emails (comma-separated).
    #[arg(short, long, value_parser = parse_to_author)]
    pub authors: Option<Authors>,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Optional path where the configuration should be initialized.
    /// Defaults to the current directory.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}
