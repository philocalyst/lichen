//! # Command Line Interface
//!
//! Defines the CLI structure and argument parsing using Clap.

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
    // |1| Attempt to parse as a full date; on Err, log only if it looks not a year
    match s.parse::<Date>() {
        Ok(date) => return Ok(date),
        Err(e) => {
            // basic assumption anything longer than 4 chars isn’t just "YYYY", and should throw an error as it is now assumed to be an attempted full date.
            if s.len() > 4 {
                return Err(e.to_string());
            }
        }
    }

    // |2| Fallback to parsing as year only
    let year: i16 = s
        .parse()
        .map_err(|e| format!("invalid year `{}`: {}", s, e))?;

    // |3| Construct January 1st of that year
    Date::new(year, 1, 1).map_err(|e| format!("invalid date: {}", e))
}

pub fn parse_to_author(input: &str) -> Result<Authors, String> {
    // If the whole string is empty or only whitespace, reject it.
    if input.trim().is_empty() {
        return Err("You need to provide at least one author in the format NAME[:EMAIL]".into());
    }

    let authors = input
        .split(',') // split entries on commas
        .map(str::trim) // trim whitespace around each entry
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            // split at most once on ':'
            let mut parts = entry.splitn(2, ':');
            let name = parts
                .next()
                .expect("splitn always yields at least one element")
                .trim();

            if name.is_empty() {
                return Err(format!("entry `{}` has empty name", entry));
            }

            // If there's a second part and it's non‑empty, use it.
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
        .collect::<Result<Vec<Author>, String>>()?; // bubble up the first error

    Ok(Authors(authors))
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
    /// Generate a license file
    Gen(GenArgs),

    /// Apply license headers to source files
    Apply(ApplyArgs),

    /// Initialize a default configuration file
    Init(InitArgs),
}

#[derive(Args, Debug)]
pub struct GenArgs {
    /// SPDX identifier of the license to generate (e.g., MIT, Apache-2.0).
    /// Can be omitted if specified in configuration.
    #[arg()]
    pub license: Option<License>,

    /// Author names and emails (In the format NAME:EMAIL; entries seperated by a comma. Email optional).
    #[arg(short, long, value_parser = parse_to_author)]
    pub authors: Option<Authors>,

    /// Enable support for multiple licenses in the same project (Default is replace)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub multiple: Option<bool>,

    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,

    /// Date for the license copyright notice (defaults to the current year).
    #[arg(short, long, value_parser = parse_year_to_date)]
    pub date: Option<Date>,
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

    /// Enable support for multiple licenses in the same project (Default is replace)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub multiple: Option<bool>,

    /// When applying headers, which kind of comment token the user *wants*
    /// Completely possible line or block doesn't exist, in which case it falls back to the other.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub prefer_block: Option<bool>,

    /// Date for the license copyright notice (defaults to the current year).
    #[arg(short, long, value_parser = parse_year_to_date)]
    pub date: Option<Date>,

    /// Regex pattern for files/directories to exclude. Applied during directory traversal.
    #[arg(short, long)] // Removed value_delimiter, regex parsing handles it
    pub exclude: Option<Regex>,

    /// Files or directories to process. Defaults to the current directory (`.`).
    #[arg(num_args = 1..)]
    pub targets: Option<Vec<PathBuf>>,

    /// Do not respect the git_ignore file (If present in directory) and other pattern defaults
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub all: Option<bool>,

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
