//! # Data Models
//!
//! Defines shared data structures used across the Lichen application.

/// Represents different types of comment tokens found in languages.
#[derive(Debug, Clone)]
pub enum CommentToken {
    /// A single-line comment token (e.g., "//", "#").
    Line(String),
    /// A block comment token pair (e.g., "/*", "*/").
    Block { start: String, end: String },
}
