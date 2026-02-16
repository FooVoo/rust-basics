//! Exercise 14: Result and_then
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use and_then() to chain Result operations
//! - Avoid nested Results
//! - Compose fallible operations

/// Parse a string and validate it's positive.
pub fn parse_positive(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| {
            if n > 0 {
                Ok(n)
            } else {
                Err(String::from("Number is not positive"))
            }
        })
}

/// Parse and divide in one chain.
pub fn parse_and_safe_divide(a: &str, b: &str) -> Result<i32, String> {
    a.parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|num_a| {
            b.parse::<i32>()
                .map_err(|e| e.to_string())
                .and_then(|num_b| {
                    if num_b == 0 {
                        Err(String::from("Division by zero"))
                    } else {
                        Ok(num_a / num_b)
                    }
                })
        })
}

/// Parse a number and check if it's even.
pub fn parse_even(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))
        .and_then(|n| {
            if n % 2 == 0 {
                Ok(n)
            } else {
                Err(format!("{} is not even", n))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert!(parse_positive("0").is_err());
        assert!(parse_positive("-5").is_err());
        assert!(parse_positive("abc").is_err());
    }

    #[test]
    fn test_parse_and_safe_divide() {
        assert_eq!(parse_and_safe_divide("10", "2"), Ok(5));
        assert!(parse_and_safe_divide("10", "0").is_err());
        assert!(parse_and_safe_divide("abc", "2").is_err());
        assert!(parse_and_safe_divide("10", "xyz").is_err());
    }

    #[test]
    fn test_parse_even() {
        assert_eq!(parse_even("42"), Ok(42));
        assert!(parse_even("43").is_err());
        assert!(parse_even("abc").is_err());
    }
}
