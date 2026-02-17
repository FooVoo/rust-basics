//! Exercise 30: Advanced monadic operations and custom types
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement custom Result-like types
//! - Create complex monadic pipelines
//! - Apply category theory concepts in practice
//! - Build composable error handling abstractions



/// A custom Result type that accumulates errors.
#[derive(Debug, PartialEq)]
pub enum Validation<T, E> {
    Success(T),
    Failure(Vec<E>),
}

impl<T, E> Validation<T, E> {
    /// Map over the success value.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Validation<U, E>  {
        todo!("Map over the success value.")
    }

    /// Apply a fallible function.
    pub fn and_then<U>(
        self,
        f: impl FnOnce(T) -> Validation<U, E>,
    ) -> Validation<U, E>  {
        todo!("Apply a fallible function.")
    }

    /// Combine two Validations, accumulating errors.
    pub fn combine<U, R>(
        self,
        other: Validation<U, E>,
        f: impl FnOnce(T, U) -> R,
    ) -> Validation<R, E>  {
        todo!("Combine two Validations, accumulating errors.")
    }
}

/// Validate that a number is positive.
pub fn validate_positive(n: i32) -> Validation<i32, String>  {
    todo!("Validate that a number is positive.")
}

/// Validate that a number is even.
pub fn validate_even(n: i32) -> Validation<i32, String>  {
    todo!("Validate that a number is even.")
}

/// Combine multiple validations, accumulating all errors.
pub fn validate_all(numbers: Vec<i32>) -> Validation<Vec<i32>, String>  {
    todo!("Combine multiple validations, accumulating all errors.")
}

/// Implement a monadic pipeline with custom error handling.
pub fn complex_pipeline(a: i32, b: i32) -> Validation<String, String>  {
    todo!("Implement a monadic pipeline with custom error handling.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_map() {
        let v: Validation<i32, String> = Validation::Success(5);
        assert_eq!(v.map(|x| x * 2), Validation::Success(10));

        let v: Validation<i32, String> = Validation::Failure(vec![String::from("error")]);
        assert_eq!(v.map(|x| x * 2), Validation::Failure(vec![String::from("error")]));
    }

    #[test]
    fn test_validation_and_then() {
        let v = Validation::Success(5);
        let result = v.and_then(|x| {
            if x > 0 {
                Validation::Success(x * 2)
            } else {
                Validation::Failure(vec![String::from("negative")])
            }
        });
        assert_eq!(result, Validation::Success(10));
    }

    #[test]
    fn test_validation_combine() {
        let v1: Validation<i32, String> = Validation::Success(3);
        let v2: Validation<i32, String> = Validation::Success(4);
        assert_eq!(
            v1.combine(v2, |a, b| a + b),
            Validation::Success(7)
        );

        let v1: Validation<i32, String> = Validation::Failure(vec![String::from("err1")]);
        let v2: Validation<i32, String> = Validation::Failure(vec![String::from("err2")]);
        assert_eq!(
            v1.combine(v2, |a, b| a + b),
            Validation::Failure(vec![String::from("err1"), String::from("err2")])
        );
    }

    #[test]
    fn test_validate_positive() {
        assert_eq!(validate_positive(5), Validation::Success(5));
        assert!(matches!(validate_positive(0), Validation::Failure(_)));
        assert!(matches!(validate_positive(-5), Validation::Failure(_)));
    }

    #[test]
    fn test_validate_even() {
        assert_eq!(validate_even(4), Validation::Success(4));
        assert!(matches!(validate_even(3), Validation::Failure(_)));
    }

    #[test]
    fn test_validate_all() {
        assert_eq!(
            validate_all(vec![2, 4, 6]),
            Validation::Success(vec![2, 4, 6])
        );
        
        match validate_all(vec![1, 2, -4]) {
            Validation::Failure(errs) => assert!(!errs.is_empty()),
            _ => panic!("Expected failure"),
        }
    }

    #[test]
    fn test_complex_pipeline() {
        assert_eq!(
            complex_pipeline(2, 4),
            Validation::Success(String::from("Result: 2 + 4 = 6"))
        );
        
        match complex_pipeline(1, 2) {
            Validation::Failure(errs) => assert!(!errs.is_empty()),
            _ => panic!("Expected failure"),
        }
    }
}
