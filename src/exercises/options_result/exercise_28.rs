//! Exercise 28: Advanced error recovery patterns
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement sophisticated error recovery
//! - Use multiple fallback strategies
//! - Build resilient error handling

/// Try multiple parsing strategies with fallbacks.
pub fn parse_resilient(s: &str) -> Result<i32, String> {
    // Try parsing as is
    s.parse::<i32>()
        .or_else(|_| {
            // Try trimming whitespace
            s.trim().parse::<i32>()
        })
        .or_else(|_| {
            // Try parsing as float and converting
            s.parse::<f64>().map(|f| f as i32)
        })
        .map_err(|e| format!("All parsing strategies failed: {}", e))
}

/// Retry operation with different parameters.
pub fn divide_with_fallback(a: i32, b: i32, fallback_b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else if fallback_b != 0 {
        Ok(a / fallback_b)
    } else {
        Err(String::from("Both divisors are zero"))
    }
}

/// Aggregate errors from multiple sources.
pub fn parse_and_aggregate(strings: &[&str]) -> Result<Vec<i32>, Vec<String>> {
    let results: Vec<_> = strings
        .iter()
        .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .collect();

    let errors: Vec<_> = results
        .iter()
        .filter_map(|r| r.as_ref().err())
        .cloned()
        .collect();

    if errors.is_empty() {
        Ok(results.into_iter().map(Result::unwrap).collect())
    } else {
        Err(errors)
    }
}

/// Recover with default on specific errors.
pub fn parse_with_default(s: &str, default: i32) -> i32 {
    s.parse::<i32>()
        .or_else(|_| {
            // If it's "default", use the default value
            if s.trim().eq_ignore_ascii_case("default") {
                Ok(default)
            } else {
                Err(())
            }
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resilient() {
        assert_eq!(parse_resilient("42"), Ok(42));
        assert_eq!(parse_resilient("  42  "), Ok(42));
        assert_eq!(parse_resilient("42.7"), Ok(42));
        assert!(parse_resilient("abc").is_err());
    }

    #[test]
    fn test_divide_with_fallback() {
        assert_eq!(divide_with_fallback(10, 2, 5), Ok(5));
        assert_eq!(divide_with_fallback(10, 0, 2), Ok(5)); // uses fallback
        assert!(divide_with_fallback(10, 0, 0).is_err());
    }

    #[test]
    fn test_parse_and_aggregate() {
        assert_eq!(
            parse_and_aggregate(&["1", "2", "3"]),
            Ok(vec![1, 2, 3])
        );
        match parse_and_aggregate(&["1", "abc", "3", "xyz"]) {
            Err(errors) => assert_eq!(errors.len(), 2),
            Ok(_) => panic!("Expected errors"),
        }
    }

    #[test]
    fn test_parse_with_default() {
        assert_eq!(parse_with_default("42", 10), 42);
        assert_eq!(parse_with_default("default", 10), 10);
        assert_eq!(parse_with_default("DEFAULT", 10), 10);
        assert_eq!(parse_with_default("xyz", 10), 0);
    }
}
