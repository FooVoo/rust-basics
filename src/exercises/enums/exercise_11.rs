//! Exercise 11: Result Patterns - Error Handling with Result<T, E>
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Work with Result<T, E> type
//! - Use map, map_err on Result
//! - Chain Result operations

/// Divides two numbers, returns Err if divisor is zero
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    todo!("Implement safe_divide")
}

/// Parses a string to i32 and doubles it
pub fn parse_and_double(s: &str) -> Result<i32, String> {
    todo!("Implement parse_and_double")
}

/// Combines two Result values with addition
pub fn add_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    todo!("Implement add_results")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(safe_divide(7.0, 2.0), Ok(3.5));
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("5"), Ok(10));
        assert_eq!(parse_and_double("0"), Ok(0));
        assert_eq!(parse_and_double("-3"), Ok(-6));
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_add_results() {
        assert_eq!(add_results(Ok(5), Ok(3)), Ok(8));
        assert_eq!(add_results(Ok(10), Ok(-5)), Ok(5));
        assert!(add_results(Err("error".to_string()), Ok(5)).is_err());
        assert!(add_results(Ok(5), Err("error".to_string())).is_err());
    }
}
