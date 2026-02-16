//! Exercise 25: Error context and propagation
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Add context to errors during propagation
//! - Chain errors with additional information
//! - Build informative error messages

/// Read a "file" (simulated) and parse its contents.
pub fn read_and_parse(filename: &str, content: &str) -> Result<i32, String> {
    content
        .parse::<i32>()
        .map_err(|e| format!("Failed to parse {}: {}", filename, e))
}

/// Process a value with context.
pub fn process_with_context(
    value: &str,
    context: &str,
) -> Result<i32, String> {
    value
        .parse::<i32>()
        .map_err(|e| format!("Error in {}: {}", context, e))
        .and_then(|n| {
            if n >= 0 {
                Ok(n)
            } else {
                Err(format!("Negative value in {}: {}", context, n))
            }
        })
}

/// Chain multiple operations with context preservation.
pub fn multi_step_with_context(
    step1: &str,
    step2: &str,
) -> Result<i32, String> {
    let value1 = step1
        .parse::<i32>()
        .map_err(|e| format!("Step 1 failed: {}", e))?;

    let value2 = step2
        .parse::<i32>()
        .map_err(|e| format!("Step 2 failed: {}", e))?;

    value1
        .checked_add(value2)
        .ok_or_else(|| String::from("Addition overflow in final step"))
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
