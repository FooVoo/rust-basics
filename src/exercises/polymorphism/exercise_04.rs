//! Exercise 04: Trait with Associated Functions - Define static methods in traits
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Define associated functions in traits
//! - Understand the difference between methods and associated functions
//! - Implement constructors via traits

pub trait Buildable {
    fn new(name: String) -> Self;
    fn default() -> Self;
}

pub struct House {
    pub name: String,
    pub rooms: u32,
}

impl Buildable for House {
    fn new(name: String) -> Self {
        todo!("Implement new")
    }
    
    fn default() -> Self {
        todo!("Implement default")
    }
}

pub struct Vehicle {
    pub name: String,
    pub wheels: u32,
}

impl Buildable for Vehicle {
    fn new(name: String) -> Self {
        todo!("Implement new")
    }
    
    fn default() -> Self {
        todo!("Implement default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_new() {
        let house = House::new("My House".to_string());
        assert_eq!(house.name, "My House");
        assert_eq!(house.rooms, 1);
    }

    #[test]
    fn test_house_default() {
        let house = House::default();
        assert_eq!(house.name, "Default House");
        assert_eq!(house.rooms, 3);
    }

    #[test]
    fn test_vehicle_new() {
        let vehicle = Vehicle::new("My Car".to_string());
        assert_eq!(vehicle.name, "My Car");
        assert_eq!(vehicle.wheels, 4);
    }

    #[test]
    fn test_vehicle_default() {
        let vehicle = Vehicle::default();
        assert_eq!(vehicle.name, "Default Vehicle");
        assert_eq!(vehicle.wheels, 4);
    }

    #[test]
    fn test_different_defaults() {
        let house = House::default();
        let vehicle = Vehicle::default();
        
        assert_ne!(house.name, vehicle.name);
    }
}
