//! Exercise 07: Generic Default - Use Default trait bound
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use Default trait bound
//! - Create default instances of generic types
//! - Initialize generic collections

/// Creates a vector with n default values.
pub fn create_defaults<T: Default>(n: usize) -> Vec<T> {
    (0..n).map(|_| T::default()).collect()
}

/// A generic wrapper that can reset to default.
pub struct Resettable<T: Default> {
    value: T,
}

impl<T: Default> Resettable<T> {
    /// Creates a new Resettable with the given value.
    pub fn new(value: T) -> Self {
        Resettable { value }
    }

    /// Creates a Resettable with the default value.
    pub fn default() -> Self {
        Resettable {
            value: T::default(),
        }
    }

    /// Returns a reference to the value.
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Sets a new value.
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    /// Resets the value to default.
    pub fn reset(&mut self) {
        self.value = T::default();
    }

    /// Consumes self and returns the value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_defaults_integers() {
        let defaults: Vec<i32> = create_defaults(3);
        assert_eq!(defaults, vec![0, 0, 0]);
    }

    #[test]
    fn test_create_defaults_strings() {
        let defaults: Vec<String> = create_defaults(2);
        assert_eq!(defaults, vec![String::new(), String::new()]);
    }

    #[test]
    fn test_create_defaults_empty() {
        let defaults: Vec<i32> = create_defaults(0);
        assert_eq!(defaults.len(), 0);
    }

    #[test]
    fn test_resettable_new() {
        let r = Resettable::new(42);
        assert_eq!(*r.get(), 42);
    }

    #[test]
    fn test_resettable_default() {
        let r: Resettable<i32> = Resettable::default();
        assert_eq!(*r.get(), 0);
    }

    #[test]
    fn test_resettable_set() {
        let mut r = Resettable::new(10);
        r.set(20);
        assert_eq!(*r.get(), 20);
    }

    #[test]
    fn test_resettable_reset() {
        let mut r = Resettable::new(100);
        r.reset();
        assert_eq!(*r.get(), 0);
    }

    #[test]
    fn test_resettable_into_inner() {
        let r = Resettable::new("hello".to_string());
        let value = r.into_inner();
        assert_eq!(value, "hello");
    }

    #[test]
    fn test_resettable_vec() {
        let mut r: Resettable<Vec<i32>> = Resettable::new(vec![1, 2, 3]);
        assert_eq!(r.get().len(), 3);
        r.reset();
        assert_eq!(r.get().len(), 0);
    }
}
