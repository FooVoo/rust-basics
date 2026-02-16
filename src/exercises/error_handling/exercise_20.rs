//! Exercise 20: Validation Pipeline - Chain multiple validations
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Chain multiple validation steps
//! - Collect all validation errors
//! - Build validation pipelines

#[derive(Debug, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

pub struct Validator<T> {
    value: T,
    errors: Vec<ValidationError>,
}

impl<T> Validator<T> {
    pub fn new(value: T) -> Self {
        Validator {
            value,
            errors: Vec::new(),
        }
    }
    
    /// Add a validation rule.
    pub fn validate<F>(mut self, field: &str, predicate: F, message: &str) -> Self
    where
        F: FnOnce(&T) -> bool,
    {
        if !predicate(&self.value) {
            self.errors.push(ValidationError::new(field, message));
        }
        self
    }
    
    /// Finalize validation.
    pub fn build(self) -> Result<T, Vec<ValidationError>> {
        if self.errors.is_empty() {
            Ok(self.value)
        } else {
            Err(self.errors)
        }
    }
}

/// Validate a product name and price.
pub fn validate_product(name: &str, price: f64) -> Result<(String, f64), Vec<ValidationError>> {
    Validator::new((name.to_string(), price))
        .validate("name", |(n, _)| !n.is_empty(), "Name cannot be empty")
        .validate("name", |(n, _)| n.len() <= 100, "Name too long")
        .validate("price", |(_, p)| *p >= 0.0, "Price cannot be negative")
        .validate("price", |(_, p)| *p <= 1_000_000.0, "Price too high")
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_all_valid() {
        let result = Validator::new(10)
            .validate("value", |v| *v > 0, "Must be positive")
            .validate("value", |v| *v < 100, "Must be less than 100")
            .build();
        
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_validator_single_error() {
        let result = Validator::new(-5)
            .validate("value", |v| *v > 0, "Must be positive")
            .build();
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "value");
    }

    #[test]
    fn test_validator_multiple_errors() {
        let result = Validator::new(-5)
            .validate("value", |v| *v > 0, "Must be positive")
            .validate("value", |v| *v < 100, "Must be less than 100")
            .validate("value", |v| *v % 2 == 0, "Must be even")
            .build();
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2); // -5 is negative and odd
    }

    #[test]
    fn test_validate_product_valid() {
        let result = validate_product("Widget", 29.99);
        assert!(result.is_ok());
        let (name, price) = result.unwrap();
        assert_eq!(name, "Widget");
        assert_eq!(price, 29.99);
    }

    #[test]
    fn test_validate_product_empty_name() {
        let result = validate_product("", 29.99);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "name"));
    }

    #[test]
    fn test_validate_product_name_too_long() {
        let long_name = "a".repeat(101);
        let result = validate_product(&long_name, 29.99);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_product_negative_price() {
        let result = validate_product("Widget", -10.0);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "price"));
    }

    #[test]
    fn test_validate_product_price_too_high() {
        let result = validate_product("Widget", 2_000_000.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_product_multiple_errors() {
        let result = validate_product("", -10.0);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2);
    }
}
