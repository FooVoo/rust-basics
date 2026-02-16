//! Exercise 17: Chaining Result combinators
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Chain multiple Result operations
//! - Build complex transformation pipelines
//! - Handle errors at each stage

/// Parse, validate range, and compute.
pub fn parse_validate_compute(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Parse error: '{}'", s))
        .and_then(|x| {
            if x >= 0 && x <= 100 {
                Ok(x)
            } else {
                Err(format!("Value {} out of range [0, 100]", x))
            }
        })
        .map(|x| x * x)
}

/// Parse two values, validate, and combine.
pub fn parse_and_max(a: &str, b: &str) -> Result<i32, String> {
    let parse_positive = |s: &str| -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|_| format!("Parse error: '{}'", s))
            .and_then(|x| {
                if x > 0 {
                    Ok(x)
                } else {
                    Err(format!("Value must be positive: {}", x))
                }
            })
    };

    parse_positive(a).and_then(|x| parse_positive(b).map(|y| x.max(y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_validate_compute() {
        assert_eq!(parse_validate_compute("5"), Ok(25));
        assert_eq!(parse_validate_compute("10"), Ok(100));
        assert!(parse_validate_compute("150").is_err());
        assert!(parse_validate_compute("-5").is_err());
        assert!(parse_validate_compute("abc").is_err());
    }

    #[test]
    fn test_parse_and_max() {
        assert_eq!(parse_and_max("5", "10"), Ok(10));
        assert_eq!(parse_and_max("20", "15"), Ok(20));
        assert!(parse_and_max("0", "10").is_err());
        assert!(parse_and_max("-5", "10").is_err());
        assert!(parse_and_max("abc", "10").is_err());
    }
}
