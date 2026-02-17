//! Exercise 11: Combining Results - Work with multiple Results
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Combine multiple Result values
//! - Use combinators like and_then, map
//! - Handle dependent operations

/// Parse three numbers and return their product.
pub fn parse_and_multiply(a: &str, b: &str, c: &str) -> Result<i32, String> {
    todo!("Implement parse_and_multiply")
}

/// Combine two Results using a combining function.
pub fn combine_results<T, E, F>(
    r1: Result<T, E>,
    r2: Result<T, E>,
    f: F,
) -> Result<T, E>
where
    F: FnOnce(T, T) -> T,
 {
    todo!("Combine two Results using a combining function.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_multiply_success() {
        assert_eq!(parse_and_multiply("2", "3", "4"), Ok(24));
        assert_eq!(parse_and_multiply("5", "5", "2"), Ok(50));
        assert_eq!(parse_and_multiply("1", "10", "10"), Ok(100));
    }

    #[test]
    fn test_parse_and_multiply_parse_error() {
        let result = parse_and_multiply("abc", "2", "3");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("First number"));
        
        let result = parse_and_multiply("1", "xyz", "3");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Second number"));
    }

    #[test]
    fn test_parse_and_multiply_overflow() {
        let result = parse_and_multiply("1000000", "1000000", "1000000");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
    }

    #[test]
    fn test_combine_results_both_ok() {
        let r1: Result<i32, String> = Ok(10);
        let r2: Result<i32, String> = Ok(20);
        assert_eq!(combine_results(r1, r2, |a, b| a + b), Ok(30));
    }

    #[test]
    fn test_combine_results_first_err() {
        let r1: Result<i32, String> = Err("error 1".to_string());
        let r2: Result<i32, String> = Ok(20);
        assert!(combine_results(r1, r2, |a, b| a + b).is_err());
    }

    #[test]
    fn test_combine_results_second_err() {
        let r1: Result<i32, String> = Ok(10);
        let r2: Result<i32, String> = Err("error 2".to_string());
        assert!(combine_results(r1, r2, |a, b| a + b).is_err());
    }

    #[test]
    fn test_combine_results_both_err() {
        let r1: Result<i32, String> = Err("error 1".to_string());
        let r2: Result<i32, String> = Err("error 2".to_string());
        let result = combine_results(r1, r2, |a, b| a + b);
        assert_eq!(result, Err("error 1".to_string()));
    }
}
