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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result  {
        todo!("Implement fmt")
    }
}

/// Safe division operation
pub fn divide(a: f64, b: f64) -> Result<f64, MathError>  {
    todo!("Safe division operation")
}

/// Safe square root operation
pub fn sqrt(x: f64) -> Result<f64, MathError>  {
    todo!("Safe square root operation")
}

/// Safe checked addition
pub fn checked_add(a: i32, b: i32) -> Result<i32, MathError>  {
    todo!("Safe checked addition")
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
