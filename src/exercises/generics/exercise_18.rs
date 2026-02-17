//! Exercise 18: Generic Validation - Build a generic validation system
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create generic validation traits
//! - Chain validations
//! - Use Result<T, E> with custom errors

/// A trait for types that can be validated.
pub trait Validator<T> {
    type Error;
    fn validate(&self, value: &T) -> Result<(), Self::Error>;
}

/// A range validator for comparable types.
pub struct RangeValidator<T> {
    min: T,
    max: T,
}

impl<T> RangeValidator<T> {
    pub fn new(min: T, max: T) -> Self {
        todo!("Implement new")
    }
}

impl<T> Validator<T> for RangeValidator<T>
where
    T: PartialOrd,
{
    type Error = String;

    fn validate(&self, value: &T) -> Result<(), Self::Error> {
        todo!("Implement validate")
    }
}

/// A length validator for types with length.
pub struct LengthValidator {
    min_len: usize,
    max_len: usize,
}

impl LengthValidator {
    pub fn new(min_len: usize, max_len: usize) -> Self {
        todo!("Implement new")
    }
}

impl<T> Validator<Vec<T>> for LengthValidator {
    type Error = String;

    fn validate(&self, value: &Vec<T>) -> Result<(), Self::Error> {
        todo!("Implement validate")
    }
}

impl Validator<String> for LengthValidator {
    type Error = String;

    fn validate(&self, value: &String) -> Result<(), Self::Error> {
        todo!("Implement validate")
    }
}

/// Validates a value with multiple validators.
pub fn validate_all<T, V>(value: &T, validators: &[V]) -> Result<(), String>
where
    V: Validator<T, Error = String>,
 {
    todo!("Validate a value with multiple validators.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_validator_valid() {
        let validator = RangeValidator::new(0, 100);
        assert!(validator.validate(&50).is_ok());
    }

    #[test]
    fn test_range_validator_below_min() {
        let validator = RangeValidator::new(10, 100);
        assert!(validator.validate(&5).is_err());
    }

    #[test]
    fn test_range_validator_above_max() {
        let validator = RangeValidator::new(0, 100);
        assert!(validator.validate(&150).is_err());
    }

    #[test]
    fn test_range_validator_boundaries() {
        let validator = RangeValidator::new(0, 100);
        assert!(validator.validate(&0).is_ok());
        assert!(validator.validate(&100).is_ok());
    }

    #[test]
    fn test_length_validator_vec_valid() {
        let validator = LengthValidator::new(2, 5);
        let vec = vec![1, 2, 3];
        assert!(validator.validate(&vec).is_ok());
    }

    #[test]
    fn test_length_validator_vec_too_short() {
        let validator = LengthValidator::new(3, 5);
        let vec = vec![1];
        assert!(validator.validate(&vec).is_err());
    }

    #[test]
    fn test_length_validator_vec_too_long() {
        let validator = LengthValidator::new(1, 3);
        let vec = vec![1, 2, 3, 4, 5];
        assert!(validator.validate(&vec).is_err());
    }

    #[test]
    fn test_length_validator_string_valid() {
        let validator = LengthValidator::new(3, 10);
        let s = "hello".to_string();
        assert!(validator.validate(&s).is_ok());
    }

    #[test]
    fn test_length_validator_string_invalid() {
        let validator = LengthValidator::new(5, 10);
        let s = "hi".to_string();
        assert!(validator.validate(&s).is_err());
    }

    #[test]
    fn test_validate_all_success() {
        let validators = vec![
            RangeValidator::new(0, 100),
            RangeValidator::new(10, 90),
        ];
        assert!(validate_all(&50, &validators).is_ok());
    }

    #[test]
    fn test_validate_all_failure() {
        let validators = vec![
            RangeValidator::new(0, 100),
            RangeValidator::new(60, 90),
        ];
        assert!(validate_all(&50, &validators).is_err());
    }
}
