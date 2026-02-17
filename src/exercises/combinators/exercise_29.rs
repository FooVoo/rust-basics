//! Exercise 29: Advanced Railway-Oriented Programming - Error accumulation
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement error accumulation
//! - Build validation systems that collect all errors
//! - Master advanced functional patterns

/// Validation result that can accumulate errors.
#[derive(Debug, Clone, PartialEq)]
pub enum Validation<T, E> {
    Success(T),
    Failure(Vec<E>),
}

impl<T, E> Validation<T, E> {
    /// Create a success validation.
    pub fn success(value: T) -> Self  {
        todo!("Create a success validation.")
    }

    /// Create a failure validation with a single error.
    pub fn failure(error: E) -> Self  {
        todo!("Create a failure validation with a single error.")
    }

    /// Map the success value.
    pub fn map<U, F>(self, f: F) -> Validation<U, E>
    where
        F: FnOnce(T) -> U,
     {
        todo!("Map the success value.")
    }

    /// Apply a function that returns a Validation.
    pub fn and_then<U, F>(self, f: F) -> Validation<U, E>
    where
        F: FnOnce(T) -> Validation<U, E>,
     {
        todo!("Apply a function that returns a Validation.")
    }

    /// Combine two validations, accumulating errors.
    pub fn combine<U>(self, other: Validation<U, E>) -> Validation<(T, U), E>  {
        todo!("Combine two validations, accumulating errors.")
    }
}

/// Validate a user registration with multiple checks.
#[derive(Debug, Clone, PartialEq)]
pub struct UserRegistration {
    pub username: String,
    pub email: String,
    pub age: i32,
}

pub fn validate_username(username: &str) -> Validation<String, String>  {
    todo!("Validate a user registration with multiple checks.")
}

pub fn validate_email(email: &str) -> Validation<String, String>  {
    todo!("Implement validate_email")
}

pub fn validate_age(age: i32) -> Validation<i32, String>  {
    todo!("Implement validate_age")
}

/// Validate complete user registration, accumulating all errors.
pub fn validate_user(
    username: &str,
    email: &str,
    age: i32,
) -> Validation<UserRegistration, String>  {
    todo!("Validate complete user registration, accumulating all errors.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_success() {
        let result = validate_user("john", "john@example.com", 25);
        assert!(matches!(result, Validation::Success(_)));
    }

    #[test]
    fn test_validation_single_error() {
        let result = validate_user("jo", "john@example.com", 25);
        match result {
            Validation::Failure(errors) => assert_eq!(errors.len(), 1),
            _ => panic!("Expected failure"),
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let result = validate_user("jo", "invalid", 16);
        match result {
            Validation::Failure(errors) => assert_eq!(errors.len(), 3),
            _ => panic!("Expected failure with 3 errors"),
        }
    }

    #[test]
    fn test_validation_combine() {
        let v1: Validation<i32, String> = Validation::success(5);
        let v2: Validation<i32, String> = Validation::success(10);
        let combined = v1.combine(v2);
        assert_eq!(combined, Validation::Success((5, 10)));
    }

    #[test]
    fn test_validation_map() {
        let v: Validation<i32, String> = Validation::success(5);
        let mapped = v.map(|x| x * 2);
        assert_eq!(mapped, Validation::Success(10));
    }
}
