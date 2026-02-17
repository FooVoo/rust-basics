//! Exercise 10: Result::and_then - Chain Result operations
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Chain Result operations
//! - Handle multiple failure points
//! - Propagate errors elegantly

/// Parse two strings and add them.
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, String> {
    a.parse::<i32>()
        .map_err(|_| format!("Failed to parse '{}'", a))
        .and_then(|x| {
            b.parse::<i32>()
                .map_err(|_| format!("Failed to parse '{}'", b))
                .map(|y| x + y)
        })
}

/// Parse and divide with error handling.
pub fn parse_and_divide(numerator: &str, denominator: &str) -> Result<i32, String> {
    numerator
        .parse::<i32>()
        .map_err(|_| format!("Invalid numerator: '{}'", numerator))
        .and_then(|n| {
            denominator
                .parse::<i32>()
                .map_err(|_| format!("Invalid denominator: '{}'", denominator))
                .and_then(|d| {
                    if d == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(n / d)
                    }
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_add() {
        assert_eq!(parse_and_add("5", "10"), Ok(15));
        assert_eq!(parse_and_add("-3", "8"), Ok(5));
        assert!(parse_and_add("abc", "10").is_err());
        assert!(parse_and_add("5", "xyz").is_err());
    }

    #[test]
    fn test_parse_and_divide() {
        assert_eq!(parse_and_divide("20", "4"), Ok(5));
        assert_eq!(parse_and_divide("15", "3"), Ok(5));
        assert!(parse_and_divide("10", "0").is_err());
        assert!(parse_and_divide("abc", "5").is_err());
        assert!(parse_and_divide("10", "xyz").is_err());
    }
}
