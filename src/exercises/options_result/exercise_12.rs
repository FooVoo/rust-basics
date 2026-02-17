//! Exercise 12: Result map and map_err
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use map() to transform Ok values
//! - Use map_err() to transform Err values
//! - Chain transformations on Result

/// Double the value if parsing succeeds.
pub fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError>  {
    todo!("Double the value if parsing succeeds.")
}

/// Parse a number and convert error to a custom message.
pub fn parse_with_custom_error(s: &str) -> Result<i32, String>  {
    todo!("Parse a number and convert error to a custom message.")
}

/// Divide and convert both result and error.
pub fn divide_verbose(a: i32, b: i32) -> Result<String, String>  {
    todo!("Divide and convert both result and error.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("21"), Ok(42));
        assert_eq!(parse_and_double("0"), Ok(0));
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_parse_with_custom_error() {
        assert_eq!(parse_with_custom_error("42"), Ok(42));
        match parse_with_custom_error("abc") {
            Err(e) => assert!(e.starts_with("Parse error:")),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_divide_verbose() {
        assert_eq!(divide_verbose(10, 2), Ok(String::from("Result: 5")));
        assert_eq!(
            divide_verbose(10, 0),
            Err(String::from("Error: Cannot divide by zero"))
        );
    }
}
