//! Exercise 15: Result or and or_else
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use or() to provide alternative Results
//! - Use or_else() for lazy alternative computation
//! - Implement fallback error handling

/// Try parsing as i32, fallback to trying as f64.
pub fn parse_number_flexible(s: &str) -> Result<i32, String> {
    todo!("Implement parse_number_flexible")
}

/// Try first operation, fallback to second if it fails.
pub fn try_with_fallback(
    first: Result<i32, String>,
    second: Result<i32, String>,
) -> Result<i32, String>  {
    todo!("Try first operation, fallback to second if it fails.")
}

/// Parse with multiple fallback attempts.
pub fn parse_with_fallbacks(s: &str) -> Result<i32, String> {
    todo!("Implement parse_with_fallbacks")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_flexible() {
        assert_eq!(parse_number_flexible("42"), Ok(42));
        assert_eq!(parse_number_flexible("42.7"), Ok(42));
        assert!(parse_number_flexible("abc").is_err());
    }

    #[test]
    fn test_try_with_fallback() {
        assert_eq!(
            try_with_fallback(Ok(1), Ok(2)),
            Ok(1)
        );
        assert_eq!(
            try_with_fallback(Err(String::from("error")), Ok(2)),
            Ok(2)
        );
        assert!(try_with_fallback(
            Err(String::from("error1")),
            Err(String::from("error2"))
        )
        .is_err());
    }

    #[test]
    fn test_parse_with_fallbacks() {
        assert_eq!(parse_with_fallbacks("42"), Ok(42));
        assert_eq!(parse_with_fallbacks("zero"), Ok(0));
        assert!(parse_with_fallbacks("abc").is_err());
    }
}
