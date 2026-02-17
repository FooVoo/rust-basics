//! Exercise 04: Safe Division - Divide two numbers safely
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Handle division by zero
//! - Return errors for invalid operations
//! - Work with floating-point numbers

/// Safely divide two numbers.
/// Return Err if dividing by zero.
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    todo!("Implement safe_divide")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide_success() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(safe_divide(15.0, 3.0), Ok(5.0));
        assert_eq!(safe_divide(7.0, 2.0), Ok(3.5));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert!(safe_divide(10.0, 0.0).is_err());
        assert_eq!(
            safe_divide(5.0, 0.0),
            Err("Division by zero".to_string())
        );
    }

    #[test]
    fn test_safe_divide_negative() {
        assert_eq!(safe_divide(-10.0, 2.0), Ok(-5.0));
        assert_eq!(safe_divide(10.0, -2.0), Ok(-5.0));
    }
}
