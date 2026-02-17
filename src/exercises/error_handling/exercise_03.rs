//! Exercise 03: Option to Result - Convert Option to Result
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Convert between Option and Result
//! - Use ok_or_else combinator
//! - Provide custom error messages

/// Convert an Option<T> to Result<T, String>.
pub fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
    todo!("Implement option_to_result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_to_result_some() {
        assert_eq!(option_to_result(Some(42), "error"), Ok(42));
        assert_eq!(option_to_result(Some("hello"), "error"), Ok("hello"));
    }

    #[test]
    fn test_option_to_result_none() {
        assert_eq!(
            option_to_result::<i32>(None, "not found"),
            Err("not found".to_string())
        );
        assert_eq!(
            option_to_result::<String>(None, "missing value"),
            Err("missing value".to_string())
        );
    }
}
