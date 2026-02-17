//! Exercise 17: Converting Result to Option
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use ok() to convert Result to Option
//! - Use err() to extract errors
//! - Understand when to discard errors

/// Convert Result to Option, discarding the error.
pub fn result_to_option(result: Result<i32, String>) -> Option<i32>  {
    todo!("Convert Result to Option, discarding the error.")
}

/// Parse a string to Option, ignoring parse errors.
pub fn parse_to_option(s: &str) -> Option<i32>  {
    todo!("Parse a string to Option, ignoring parse errors.")
}

/// Extract the error from a Result as an Option.
pub fn get_error(result: Result<i32, String>) -> Option<String>  {
    todo!("Extract the error from a Result as an Option.")
}

/// Try parsing and return Ok values only.
pub fn collect_valid_numbers(strings: &[&str]) -> Vec<i32>  {
    todo!("Try parsing and return Ok values only.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_to_option() {
        assert_eq!(result_to_option(Ok(42)), Some(42));
        assert_eq!(result_to_option(Err(String::from("error"))), None);
    }

    #[test]
    fn test_parse_to_option() {
        assert_eq!(parse_to_option("42"), Some(42));
        assert_eq!(parse_to_option("abc"), None);
    }

    #[test]
    fn test_get_error() {
        assert_eq!(get_error(Ok::<i32, String>(42)), None);
        assert_eq!(
            get_error(Err(String::from("error"))),
            Some(String::from("error"))
        );
    }

    #[test]
    fn test_collect_valid_numbers() {
        assert_eq!(
            collect_valid_numbers(&["1", "2", "abc", "3", "def"]),
            vec![1, 2, 3]
        );
        assert_eq!(collect_valid_numbers(&["abc", "def"]), vec![]);
        assert_eq!(collect_valid_numbers(&["1", "2", "3"]), vec![1, 2, 3]);
    }
}
