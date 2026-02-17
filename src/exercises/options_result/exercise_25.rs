//! Exercise 25: Error context and propagation
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Add context to errors during propagation
//! - Chain errors with additional information
//! - Build informative error messages

/// Read a "file" (simulated) and parse its contents.
pub fn read_and_parse(filename: &str, content: &str) -> Result<i32, String>  {
    todo!("Read a \"file\" (simulated) and parse its contents.")
}

/// Process a value with context.
pub fn process_with_context(
    value: &str,
    context: &str,
) -> Result<i32, String>  {
    todo!("Process a value with context.")
}

/// Chain multiple operations with context preservation.
pub fn multi_step_with_context(
    step1: &str,
    step2: &str,
) -> Result<i32, String>  {
    todo!("Chain multiple operations with context preservation.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_parse() {
        assert_eq!(read_and_parse("data.txt", "42"), Ok(42));
        match read_and_parse("config.txt", "abc") {
            Err(e) => assert!(e.contains("config.txt")),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_process_with_context() {
        assert_eq!(process_with_context("42", "user input"), Ok(42));
        match process_with_context("abc", "user input") {
            Err(e) => assert!(e.contains("user input")),
            Ok(_) => panic!("Expected error"),
        }
        match process_with_context("-5", "validation") {
            Err(e) => assert!(e.contains("validation")),
            Ok(_) => panic!("Expected error"),
        }
    }

    #[test]
    fn test_multi_step_with_context() {
        assert_eq!(multi_step_with_context("5", "10"), Ok(15));
        match multi_step_with_context("abc", "10") {
            Err(e) => assert!(e.contains("Step 1")),
            Ok(_) => panic!("Expected error"),
        }
        match multi_step_with_context("10", "abc") {
            Err(e) => assert!(e.contains("Step 2")),
            Ok(_) => panic!("Expected error"),
        }
    }
}
