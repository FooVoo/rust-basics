//! Exercise 06: Error Propagation - Using the ? operator
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use the ? operator for error propagation
//! - Chain multiple fallible operations
//! - Understand early return on errors

/// Parse two numbers from strings and return their sum.
/// Use the ? operator to propagate errors.
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, String> {
    let num_a = a.parse::<i32>().map_err(|e| e.to_string())?;
    let num_b = b.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(num_a + num_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_add_success() {
        assert_eq!(parse_and_add("10", "20"), Ok(30));
        assert_eq!(parse_and_add("0", "0"), Ok(0));
        assert_eq!(parse_and_add("-5", "10"), Ok(5));
        assert_eq!(parse_and_add("100", "-50"), Ok(50));
    }

    #[test]
    fn test_parse_and_add_first_invalid() {
        assert!(parse_and_add("abc", "20").is_err());
        assert!(parse_and_add("", "10").is_err());
    }

    #[test]
    fn test_parse_and_add_second_invalid() {
        assert!(parse_and_add("10", "xyz").is_err());
        assert!(parse_and_add("20", "").is_err());
    }

    #[test]
    fn test_parse_and_add_both_invalid() {
        assert!(parse_and_add("abc", "xyz").is_err());
        assert!(parse_and_add("", "").is_err());
    }
}
