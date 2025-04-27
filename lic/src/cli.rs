//! # Command Line Interface
//!
//! Defines the CLI struct and how argument parsing is handled with Clap.

use crate::config::{Author, Authors};
use crate::license::License;
use clap::{Args, ColorChoice, Parser, Subcommand, builder::styling};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use jiff::civil::Date;
use regex::Regex;
use std::path::PathBuf;

// ▰▰▰ CLI Argument Structs ▰▰▰ //

fn parse_year_to_date(s: &str) -> Result<Date, String> {
    println!("{s}");
    // Attempt to parse as a full date; If parsing fails, it's only a problem if it was *meant* to be a full-date.
    match s.parse::<Date>() {
        Ok(date) => return Ok(date),
        Err(e) => {
            // Basic assumption here is that anything longer than 4 chars, the modern length for a year, is an incorrectly formatted date string.
            if s.len() > 4 {
                return Err(e.to_string());
            }
        }
    }

    // Fallback to parsing as year only. This fails if there are any non-numbers in the string
    let year: i16 = s
        .parse()
        .map_err(|e| format!("invalid year `{}`: {} Please use only numerals", s, e))?;

    // Construct to a chill, January 1st of that year.
    Date::new(year, 1, 1).map_err(|e| format!("invalid date: {}", e))
}

pub fn parse_to_author(input: &str) -> Result<Authors, String> {
    // If the whole string is only whitespace, reject it.
    if input.trim().is_empty() {
        return Err("You need to provide at least one author in the format NAME[:EMAIL]".into());
    }

    let authors = input
        .split(',') // Delimter for consecutive authors is a comma
        .map(str::trim) // Whitespace is bad
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            // Split at most once on ':'
            let mut parts = entry.splitn(2, ':');
            let name = parts
                .next()
                .expect("splitn always yields at least one element")
                .trim();

            if name.is_empty() {
                return Err(format!("entry `{}` has empty name", entry));
            }

            // If an email exists, and isn't blank, use it.
            let email = parts
                .next()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string);

            Ok(Author {
                name: name.to_string(),
                email,
            })
        })
        .collect::<Result<Vec<Author>, String>>()?; // Fails immeidately with any errors.

    Ok(Authors(authors))
}

// The styles I got from the docs lol
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

    /// A configuration file. Defaults to a ".lichen.toml" in the current dir
    #[arg(long, short)]
    pub config: Option<PathBuf>,
}

// Common arguments related to license information
#[derive(Args, Debug)]
pub struct LicenseArgs {
    /// SPDX identifier of the license to generate (e.g., MIT, Apache-2.0).
    /// Can be omitted if specified in configuration.
    #[arg()]
    pub license: Option<License>,

    /// Author names and emails (In the format NAME:EMAIL; entries seperated by a comma. Email optional).
    #[arg(short, long, value_parser = parse_to_author)]
    pub authors: Option<Authors>,

    /// Date for the license copyright notice (defaults to the current year).
    #[arg(short, long, value_parser = parse_year_to_date)]
    pub date: Option<Date>,

    /// Enable support for multiple licenses in the same project (Default is replace)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub multiple: Option<bool>,
}

// Common arguments for file processing
#[derive(Args, Debug)]
pub struct FileProcessingArgs {
    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,

    /// Regex pattern for files/directories to exclude. Applied during directory traversal.
    #[arg(short, long)]
    pub exclude: Option<Regex>,

    /// Do not respect the git_ignore file (If present in directory) and other pattern defaults
    #[arg(short = 'A', long, action = clap::ArgAction::SetTrue)]
    pub all: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a license file
    Gen(GenArgs),

    /// Remove license headers in source files
    Unapply(UnapplyArgs),

    /// Apply license headers to source files
    Apply(ApplyArgs),

    /// Initialize a default configuration file
    Init(InitArgs),
}

#[derive(Args, Debug)]
pub struct GenArgs {
    #[command(flatten)]
    pub license_args: LicenseArgs,

    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,
}

#[derive(Args, Debug)]
pub struct ApplyArgs {
    #[command(flatten)]
    pub license_args: LicenseArgs,

    #[command(flatten)]
    pub file_args: FileProcessingArgs,

    /// Run without modification. See what would be changed.
    #[arg(short = 'D', long, action = clap::ArgAction::SetTrue)]
    pub dry_run: Option<bool>,

    /// When applying headers, which kind of comment token the user *wants*
    /// Completely possible line or block doesn't exist, in which case it falls back to the other.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub prefer_block: Option<bool>,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Optional path where the configuration should be initialized.
    /// Defaults to the current directory.
    #[arg()]
    pub target: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct UnapplyArgs {
    #[command(flatten)]
    pub file_args: FileProcessingArgs,

    /// Run without modification. See what would be changed.
    #[arg(short = 'D', long)]
    pub dry_run: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_year_to_date_valid() {
        let d = parse_year_to_date("2023").expect("should parse");
        assert_eq!(d.year(), 2023);
        assert_eq!(d.month(), 1);
        assert_eq!(d.day(), 1);
    }

    #[test]
    fn test_parse_year_to_date_invalid() {
        let err = parse_year_to_date("abcd").unwrap_err();
        assert!(
            err.contains("invalid year"),
            "expected invalid-year error, got `{}`",
            err
        );
    }

    #[test]
    fn test_parse_to_author_single_name() {
        let authors = parse_to_author("Alice").expect("should parse");
        let list: Vec<_> = authors.0.iter().collect();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "Alice");
        assert!(list[0].email.is_none());
    }

    #[test]
    fn test_parse_to_author_name_email_and_spaces() {
        let s = "Bob: bob@example.com , Carol:carol@x.org";
        let authors = parse_to_author(s).expect("should parse");
        let list = &authors.0;
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "Bob");
        assert_eq!(list[0].email.as_deref(), Some("bob@example.com"));
        assert_eq!(list[1].name, "Carol");
        assert_eq!(list[1].email.as_deref(), Some("carol@x.org"));
    }

    #[test]
    fn test_parse_to_author_empty_input() {
        let err = parse_to_author("   ").unwrap_err();
        assert!(err.contains("You need to provide at least one author"));
    }

    #[test]
    fn test_parse_to_author_missing_name() {
        let err = parse_to_author(":no_name@example.com").unwrap_err();
        assert!(err.contains("has empty name"));
    }
}
