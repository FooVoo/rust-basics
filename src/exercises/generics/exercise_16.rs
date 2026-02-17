//! Exercise 16: Generic Builder Pattern - Implement a generic builder
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use generics with builder pattern
//! - Implement method chaining with generics
//! - Work with Option<T> in builders

/// A generic builder for creating configured objects.
pub struct Builder<T, U> {
    name: Option<String>,
    value: Option<T>,
    metadata: Option<U>,
}

impl<T, U> Builder<T, U> {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        todo!("Implement new")
    }

    /// Sets the name.
    pub fn name(mut self, name: String) -> Self {
        todo!("Implement name")
    }

    /// Sets the value.
    pub fn value(mut self, value: T) -> Self {
        todo!("Implement value")
    }

    /// Sets the metadata.
    pub fn metadata(mut self, metadata: U) -> Self {
        todo!("Implement metadata")
    }

    /// Builds the final object.
    pub fn build(self) -> Result<Built<T, U>, String> {
        todo!("Implement build")
    }
}

impl<T, U> Default for Builder<T, U> {
    fn default() -> Self {
        todo!("Implement default")
    }
}

/// The final built object.
pub struct Built<T, U> {
    pub name: String,
    pub value: T,
    pub metadata: Option<U>,
}

impl<T, U> Built<T, U> {
    /// Gets the name.
    pub fn name(&self) -> &str {
        todo!("Implement name")
    }

    /// Gets a reference to the value.
    pub fn value(&self) -> &T {
        todo!("Implement value")
    }

    /// Gets a reference to the metadata if present.
    pub fn metadata(&self) -> Option<&U> {
        todo!("Implement metadata")
    }

    /// Checks if metadata is present.
    pub fn has_metadata(&self) -> bool {
        todo!("Implement has_metadata")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_complete() {
        let result = Builder::new()
            .name("test".to_string())
            .value(42)
            .metadata("info".to_string())
            .build();
        
        assert!(result.is_ok());
        let built = result.unwrap();
        assert_eq!(built.name(), "test");
        assert_eq!(*built.value(), 42);
        assert!(built.has_metadata());
    }

    #[test]
    fn test_builder_without_metadata() {
        let result: Result<Built<i32, String>, String> = Builder::new()
            .name("test".to_string())
            .value(42)
            .build();
        
        assert!(result.is_ok());
        let built = result.unwrap();
        assert!(!built.has_metadata());
    }

    #[test]
    fn test_builder_missing_name() {
        let result: Result<Built<i32, String>, String> = Builder::new()
            .value(42)
            .build();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_missing_value() {
        let result: Result<Built<i32, String>, String> = Builder::new()
            .name("test".to_string())
            .build();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_different_types() {
        let result = Builder::new()
            .name("vector".to_string())
            .value(vec![1, 2, 3])
            .metadata(Some(42))
            .build();
        
        assert!(result.is_ok());
        let built = result.unwrap();
        assert_eq!(built.value().len(), 3);
    }

    #[test]
    fn test_builder_chain() {
        let builder: Builder<String, i32> = Builder::new();
        let result = builder
            .name("chained".to_string())
            .value("test".to_string())
            .metadata(100)
            .build();
        
        assert!(result.is_ok());
    }
}
