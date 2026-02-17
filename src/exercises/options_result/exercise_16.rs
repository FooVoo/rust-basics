//! Exercise 16: Converting Option to Result
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use ok_or() to convert Option to Result
//! - Use ok_or_else() for lazy error construction
//! - Understand when to convert between types

/// Convert an Option to Result with a static error message.
pub fn option_to_result(opt: Option<i32>) -> Result<i32, String>  {
    todo!("Convert an Option to Result with a static error message.")
}

/// Convert Option to Result with computed error.
pub fn option_to_result_lazy(opt: Option<String>) -> Result<String, String>  {
    todo!("Convert Option to Result with computed error.")
}

/// Find an element and convert to Result.
pub fn find_or_error(numbers: &[i32], target: i32) -> Result<usize, String>  {
    todo!("Find an element and convert to Result.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_to_result() {
        assert_eq!(option_to_result(Some(42)), Ok(42));
        assert_eq!(
            option_to_result(None),
            Err(String::from("Value is None"))
        );
    }

    #[test]
    fn test_option_to_result_lazy() {
        assert_eq!(
            option_to_result_lazy(Some(String::from("hello"))),
            Ok(String::from("hello"))
        );
        assert_eq!(
            option_to_result_lazy(None),
            Err(String::from("No value provided"))
        );
    }

    #[test]
    fn test_find_or_error() {
        assert_eq!(find_or_error(&[1, 2, 3, 4], 3), Ok(2));
        assert!(find_or_error(&[1, 2, 3], 5).is_err());
    }
}
