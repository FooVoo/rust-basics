//! Exercise 23: Advanced combinator chains
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Chain multiple combinators efficiently
//! - Build complex transformation pipelines
//! - Optimize combinator usage

/// Parse, validate, and transform in one chain.
pub fn parse_validate_transform(s: &str) -> Result<String, String> {
    s.parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| {
            if n >= 0 {
                Ok(n)
            } else {
                Err(String::from("Negative number"))
            }
        })
        .map(|n| n * 2)
        .map(|n| format!("Result: {}", n))
}

/// Complex option chain with multiple transformations.
pub fn complex_option_chain(s: &str) -> Option<usize> {
    s.chars()
        .nth(0)
        .filter(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .map(|d| d as usize)
        .filter(|&n| n > 0)
        .map(|n| n * 10)
}

/// Chain multiple Result operations with different transformations.
pub fn result_pipeline(a: &str, b: &str) -> Result<f64, String> {
    let num_a = a.parse::<i32>().map_err(|e| e.to_string())?;
    let num_b = b.parse::<i32>().map_err(|e| e.to_string())?;

    if num_b == 0 {
        return Err(String::from("Division by zero"));
    }

    Ok((num_a as f64 / num_b as f64).abs())
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
