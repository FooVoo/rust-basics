//! Exercise 09: Custom Error Types - Define custom error enums
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Define custom error enums
//! - Implement Display trait for errors
//! - Use custom errors in functions

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Implement fmt")
    }
}

impl std::error::Error for MathError {}

/// Calculate the square root of a number.
/// Return MathError::NegativeSquareRoot if input is negative.
pub fn sqrt(x: f64) -> Result<f64, MathError> {
    todo!("Implement sqrt")
}

/// Divide two numbers, checking for overflow.
/// Return appropriate MathError on failure.
pub fn checked_divide(a: i32, b: i32) -> Result<i32, MathError> {
    todo!("Implement checked_divide")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_valid() {
        assert_eq!(sqrt(4.0), Ok(2.0));
        assert_eq!(sqrt(9.0), Ok(3.0));
        assert_eq!(sqrt(0.0), Ok(0.0));
    }

    #[test]
    fn test_sqrt_negative() {
        assert_eq!(sqrt(-1.0), Err(MathError::NegativeSquareRoot));
        assert_eq!(sqrt(-100.0), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_checked_divide_success() {
        assert_eq!(checked_divide(10, 2), Ok(5));
        assert_eq!(checked_divide(-10, 2), Ok(-5));
        assert_eq!(checked_divide(7, 2), Ok(3));
    }

    #[test]
    fn test_checked_divide_by_zero() {
        assert_eq!(checked_divide(10, 0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_checked_divide_overflow() {
        assert_eq!(checked_divide(i32::MIN, -1), Err(MathError::Overflow));
    }

    #[test]
    fn test_error_display() {
        assert_eq!(MathError::DivisionByZero.to_string(), "Cannot divide by zero");
        assert_eq!(
            MathError::NegativeSquareRoot.to_string(),
            "Cannot take square root of negative number"
        );
    }
}
