//! Exercise 19: Custom error types with Result
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Define custom error types
//! - Implement Display and Debug for errors
//! - Use custom errors with Result

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::NegativeSquareRoot => write!(f, "Square root of negative number"),
            MathError::Overflow => write!(f, "Arithmetic overflow"),
        }
    }
}

/// Divide two numbers with custom error.
pub fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Check if square root is possible (non-negative).
pub fn sqrt_check(n: i32) -> Result<i32, MathError> {
    if n < 0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(n)
    }
}

/// Checked addition that returns custom error on overflow.
pub fn checked_add(a: i32, b: i32) -> Result<i32, MathError> {
    a.checked_add(b).ok_or(MathError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(10, 0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_sqrt_check() {
        assert_eq!(sqrt_check(4), Ok(4));
        assert_eq!(sqrt_check(-1), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(1, 2), Ok(3));
        assert_eq!(checked_add(i32::MAX, 1), Err(MathError::Overflow));
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", MathError::DivisionByZero),
            "Division by zero"
        );
        assert_eq!(
            format!("{}", MathError::NegativeSquareRoot),
            "Square root of negative number"
        );
    }
}
