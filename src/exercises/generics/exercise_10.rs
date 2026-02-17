//! Exercise 10: Generic Result Patterns - Work with Result<T, E>
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Master Result<T, E> with generics
//! - Handle errors with generic types
//! - Implement custom result types

/// A generic function that safely divides two numbers.
pub fn safe_divide<T>(a: T, b: T) -> Result<T, String>
where
    T: std::ops::Div<Output = T> + PartialEq + Default + std::fmt::Display + Copy,
 {
    todo!("A generic function that safely divides two numbers.")
}

/// A generic function that parses a string to any FromStr type.
pub fn parse_value<T>(s: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
 {
    todo!("A generic function that parses a string to any FromStr type.")
}

/// A custom Result type with detailed errors.
pub enum Outcome<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> Outcome<T, E> {
    /// Maps an Outcome<T, E> to Outcome<U, E> by applying a function to Success.
    pub fn map<U, F>(self, f: F) -> Outcome<U, E>
    where
        F: FnOnce(T) -> U,
     {
        todo!("Maps an Outcome<T, E> to Outcome<U, E> by applying a function to Success.")
    }

    /// Returns the success value or a default.
    pub fn unwrap_or(self, default: T) -> T  {
        todo!("Return the success value or a default.")
    }

    /// Returns true if the outcome is Success.
    pub fn is_success(&self) -> bool  {
        todo!("Return true if the outcome is Success.")
    }

    /// Returns true if the outcome is Failure.
    pub fn is_failure(&self) -> bool  {
        todo!("Return true if the outcome is Failure.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide_success() {
        let result = safe_divide(10, 2);
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        let result = safe_divide(10, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_divide_floats() {
        let result = safe_divide(10.0, 2.0);
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn test_parse_value_success() {
        let result: Result<i32, String> = parse_value("42");
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_parse_value_failure() {
        let result: Result<i32, String> = parse_value("not a number");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_value_float() {
        let result: Result<f64, String> = parse_value("3.14");
        assert_eq!(result, Ok(3.14));
    }

    #[test]
    fn test_outcome_success() {
        let outcome: Outcome<i32, String> = Outcome::Success(42);
        assert!(outcome.is_success());
        assert!(!outcome.is_failure());
    }

    #[test]
    fn test_outcome_failure() {
        let outcome: Outcome<i32, String> = Outcome::Failure("error".to_string());
        assert!(outcome.is_failure());
        assert!(!outcome.is_success());
    }

    #[test]
    fn test_outcome_map() {
        let outcome: Outcome<i32, String> = Outcome::Success(5);
        let result = outcome.map(|x| x * 2);
        assert_eq!(result.unwrap_or(0), 10);
    }

    #[test]
    fn test_outcome_map_failure() {
        let outcome: Outcome<i32, String> = Outcome::Failure("error".to_string());
        let result = outcome.map(|x| x * 2);
        assert!(result.is_failure());
    }
}
