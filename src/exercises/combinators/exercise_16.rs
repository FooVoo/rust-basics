//! Exercise 16: Option::transpose - Convert Option<Result> to Result<Option>
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use transpose to swap Option and Result
//! - Handle optional operations that can fail
//! - Understand type transformations

/// Parse only if string is not empty.
pub fn parse_non_empty(s: &str) -> Result<Option<i32>, std::num::ParseIntError>  {
    todo!("Parse only if string is not empty.")
}

/// Get and parse first element.
pub fn parse_first(strings: &[&str]) -> Result<Option<i32>, std::num::ParseIntError>  {
    todo!("Get and parse first element.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_non_empty() {
        assert_eq!(parse_non_empty("42"), Ok(Some(42)));
        assert_eq!(parse_non_empty(""), Ok(None));
        assert!(parse_non_empty("abc").is_err());
    }

    #[test]
    fn test_parse_first() {
        assert_eq!(parse_first(&["10", "20"]), Ok(Some(10)));
        assert_eq!(parse_first(&[]), Ok(None));
        assert!(parse_first(&["abc", "20"]).is_err());
    }
}
