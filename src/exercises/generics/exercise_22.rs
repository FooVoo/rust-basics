//! Exercise 22: PhantomData - Use PhantomData for zero-sized type parameters
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand PhantomData usage
//! - Create type-safe markers
//! - Implement phantom types for compile-time guarantees

use std::marker::PhantomData;

/// Type-level markers for different states.
pub struct Locked;
pub struct Unlocked;

/// A generic container with a phantom type for state.
pub struct SecureContainer<T, State> {
    data: T,
    _state: PhantomData<State>,
}

impl<T> SecureContainer<T, Locked> {
    /// Creates a new locked container.
    pub fn new(data: T) -> Self {
        todo!("Implement new")
    }

    /// Unlocks the container.
    pub fn unlock(self, _key: &str) -> SecureContainer<T, Unlocked> {
        todo!("Implement unlock")
    }
}

impl<T> SecureContainer<T, Unlocked> {
    /// Gets a reference to the data (only available when unlocked).
    pub fn get(&self) -> &T {
        todo!("Implement get")
    }

    /// Gets a mutable reference to the data (only available when unlocked).
    pub fn get_mut(&mut self) -> &mut T {
        todo!("Implement get_mut")
    }

    /// Locks the container.
    pub fn lock(self) -> SecureContainer<T, Locked> {
        todo!("Implement lock")
    }

    /// Consumes the container and returns the data.
    pub fn into_inner(self) -> T {
        todo!("Implement into_inner")
    }
}

/// Markers for different units.
pub struct Meters;
pub struct Feet;

/// A generic measurement with a phantom unit type.
pub struct Measurement<T, Unit> {
    value: T,
    _unit: PhantomData<Unit>,
}

impl<T> Measurement<T, Meters> {
    /// Creates a measurement in meters.
    pub fn meters(value: T) -> Self {
        todo!("Implement meters")
    }
}

impl<T> Measurement<T, Feet> {
    /// Creates a measurement in feet.
    pub fn feet(value: T) -> Self {
        todo!("Implement feet")
    }
}

impl<T, Unit> Measurement<T, Unit> {
    /// Gets the raw value.
    pub fn value(&self) -> &T {
        todo!("Implement value")
    }
}

impl Measurement<f64, Meters> {
    /// Converts meters to feet.
    pub fn to_feet(self) -> Measurement<f64, Feet> {
        todo!("Implement to_feet")
    }
}

impl Measurement<f64, Feet> {
    /// Converts feet to meters.
    pub fn to_meters(self) -> Measurement<f64, Meters> {
        todo!("Implement to_meters")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_container_locked() {
        let container = SecureContainer::<i32, Locked>::new(42);
        // Cannot call get() on locked container - compile-time safety
        let _unlocked = container.unlock("key");
    }

    #[test]
    fn test_secure_container_unlock() {
        let container = SecureContainer::new(42);
        let unlocked = container.unlock("secret");
        assert_eq!(*unlocked.get(), 42);
    }

    #[test]
    fn test_secure_container_lock() {
        let container = SecureContainer::new(42);
        let unlocked = container.unlock("key");
        let _locked = unlocked.lock();
    }

    #[test]
    fn test_secure_container_get_mut() {
        let container = SecureContainer::new(42);
        let mut unlocked = container.unlock("key");
        *unlocked.get_mut() = 100;
        assert_eq!(*unlocked.get(), 100);
    }

    #[test]
    fn test_secure_container_into_inner() {
        let container = SecureContainer::new(vec![1, 2, 3]);
        let unlocked = container.unlock("key");
        let data = unlocked.into_inner();
        assert_eq!(data, vec![1, 2, 3]);
    }

    #[test]
    fn test_measurement_meters() {
        let m = Measurement::meters(10.0);
        assert_eq!(*m.value(), 10.0);
    }

    #[test]
    fn test_measurement_feet() {
        let f = Measurement::feet(10.0);
        assert_eq!(*f.value(), 10.0);
    }

    #[test]
    fn test_meters_to_feet() {
        let m = Measurement::meters(1.0);
        let f = m.to_feet();
        assert!((*f.value() - 3.28084).abs() < 0.0001);
    }

    #[test]
    fn test_feet_to_meters() {
        let f = Measurement::feet(3.28084);
        let m = f.to_meters();
        assert!((*m.value() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_conversion_roundtrip() {
        let m1 = Measurement::meters(10.0);
        let f = m1.to_feet();
        let m2 = f.to_meters();
        assert!((*m2.value() - 10.0).abs() < 0.001);
    }
}
