//! Exercise 16: Custom Error Types - Building Error Enums
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create custom error types using enums
//! - Implement Display and Error traits
//! - Use custom errors in Result types

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
            MathError::Overflow => write!(f, "Arithmetic overflow occurred"),
        }
    }
}

/// Safe division operation
pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Safe square root operation
pub fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

/// Safe checked addition
pub fn checked_add(a: i32, b: i32) -> Result<i32, MathError> {
    a.checked_add(b).ok_or(MathError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(10.0, 0.0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(4.0), Ok(2.0));
        assert_eq!(sqrt(0.0), Ok(0.0));
        assert_eq!(sqrt(-1.0), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_checked_add() {
        assert_eq!(checked_add(5, 10), Ok(15));
        assert_eq!(checked_add(i32::MAX, 1), Err(MathError::Overflow));
    }

    #[test]
    fn test_display() {
        assert_eq!(
            MathError::DivisionByZero.to_string(),
            "Cannot divide by zero"
        );
        assert_eq!(
            MathError::NegativeSquareRoot.to_string(),
            "Cannot take square root of negative number"
        );
    }
}
