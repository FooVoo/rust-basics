//! Exercise 23: Advanced combinator chains
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Chain multiple combinators efficiently
//! - Build complex transformation pipelines
//! - Optimize combinator usage

/// Parse, validate, and transform in one chain.
pub fn parse_validate_transform(s: &str) -> Result<String, String> {
    todo!("Implement parse_validate_transform")
}

/// Complex option chain with multiple transformations.
pub fn complex_option_chain(s: &str) -> Option<usize> {
    todo!("Implement complex_option_chain")
}

/// Chain multiple Result operations with different transformations.
pub fn result_pipeline(a: &str, b: &str) -> Result<f64, String> {
    todo!("Implement result_pipeline")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_validate_transform() {
        assert_eq!(
            parse_validate_transform("5"),
            Ok(String::from("Result: 10"))
        );
        assert_eq!(
            parse_validate_transform("0"),
            Ok(String::from("Result: 0"))
        );
        assert!(parse_validate_transform("-5").is_err());
        assert!(parse_validate_transform("abc").is_err());
    }

    #[test]
    fn test_complex_option_chain() {
        assert_eq!(complex_option_chain("5hello"), Some(50));
        assert_eq!(complex_option_chain("0hello"), None); // filtered out (not > 0)
        assert_eq!(complex_option_chain("abc"), None);
        assert_eq!(complex_option_chain(""), None);
    }

    #[test]
    fn test_result_pipeline() {
        assert_eq!(result_pipeline("10", "2"), Ok(5.0));
        assert_eq!(result_pipeline("-10", "2"), Ok(5.0)); // abs
        assert!(result_pipeline("10", "0").is_err());
        assert!(result_pipeline("abc", "2").is_err());
    }
}
