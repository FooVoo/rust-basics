//! Exercise 13: The ? operator basics
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use the ? operator for error propagation
//! - Understand early return with ?
//! - Chain multiple ? operations

/// Parse two strings and add them using ?.
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let num_a = a.parse::<i32>()?;
    let num_b = b.parse::<i32>()?;
    Ok(num_a + num_b)
}

/// Parse three strings and multiply them.
pub fn parse_and_multiply(
    a: &str,
    b: &str,
    c: &str,
) -> Result<i32, std::num::ParseIntError> {
    let num_a = a.parse::<i32>()?;
    let num_b = b.parse::<i32>()?;
    let num_c = c.parse::<i32>()?;
    Ok(num_a * num_b * num_c)
}

/// Divide two parsed numbers.
pub fn parse_and_divide(a: &str, b: &str) -> Result<i32, String> {
    let num_a = a.parse::<i32>().map_err(|e| e.to_string())?;
    let num_b = b.parse::<i32>().map_err(|e| e.to_string())?;
    if num_b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(num_a / num_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_add() {
        assert_eq!(parse_and_add("5", "10"), Ok(15));
        assert_eq!(parse_and_add("0", "0"), Ok(0));
        assert!(parse_and_add("abc", "10").is_err());
        assert!(parse_and_add("5", "xyz").is_err());
    }

    #[test]
    fn test_parse_and_multiply() {
        assert_eq!(parse_and_multiply("2", "3", "4"), Ok(24));
        assert_eq!(parse_and_multiply("0", "5", "10"), Ok(0));
        assert!(parse_and_multiply("a", "2", "3").is_err());
    }

    #[test]
    fn test_parse_and_divide() {
        assert_eq!(parse_and_divide("10", "2"), Ok(5));
        assert!(parse_and_divide("10", "0").is_err());
        assert!(parse_and_divide("abc", "2").is_err());
    }
}
