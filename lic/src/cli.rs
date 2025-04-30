//! # Command Line Interface
//!
//! Defines unit tests

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
