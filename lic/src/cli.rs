//! # Command Line Interface
//!
//! Defines the CLI struct and how argument parsing is handled with Clap.

use crate::models::Commands;
use clap::{ColorChoice, Parser, builder::styling};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;

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

#[cfg(test)]
mod tests {
    use crate::models::{parse_to_author, parse_year_to_date};

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
