//! Exercise 23: Railway-oriented programming - Success/Failure tracks
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand railway-oriented programming
//! - Build validation pipelines
//! - Chain validations elegantly

/// Validation function type.
type Validator<T> = fn(T) -> Result<T, String>;

/// Compose multiple validators into a pipeline.
pub fn validate_pipeline<T: Clone>(
    value: T,
    validators: Vec<Validator<T>>,
) -> Result<T, String> {
    validators
        .into_iter()
        .try_fold(value, |val, validator| validator(val))
}

/// Validate number is positive.
pub fn validate_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err(format!("Must be positive, got {}", n))
    }
}

/// Validate number is even.
pub fn validate_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(format!("Must be even, got {}", n))
    }
}

/// Validate number is less than 100.
pub fn validate_less_than_100(n: i32) -> Result<i32, String> {
    if n < 100 {
        Ok(n)
    } else {
        Err(format!("Must be less than 100, got {}", n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_pipeline_success() {
        let validators = vec![validate_positive, validate_even, validate_less_than_100];
        assert_eq!(validate_pipeline(10, validators), Ok(10));
    }

    #[test]
    fn test_validate_pipeline_failure() {
        let validators = vec![validate_positive, validate_even];
        assert!(validate_pipeline(-5, validators).is_err());

        let validators = vec![validate_positive, validate_even];
        assert!(validate_pipeline(7, validators).is_err());
    }

    #[test]
    fn test_individual_validators() {
        assert_eq!(validate_positive(5), Ok(5));
        assert!(validate_positive(-5).is_err());

        assert_eq!(validate_even(4), Ok(4));
        assert!(validate_even(3).is_err());

        assert_eq!(validate_less_than_100(50), Ok(50));
        assert!(validate_less_than_100(150).is_err());
    }
}
