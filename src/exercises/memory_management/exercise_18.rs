//! Exercise 18: Lifetime Bounds - Generic types with lifetime constraints
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Combine lifetimes with generic types
//! - Use lifetime bounds in trait implementations
//! - Understand T: 'a syntax

pub struct Container<'a, T> {
    data: &'a T,
}

impl<'a, T> Container<'a, T> {
    pub fn new(data: &'a T) -> Self {
        todo!("Implement new")
    }
    
    pub fn get(&self) -> &T {
        todo!("Implement get")
    }
}

/// Create a container with the given data.
pub fn make_container<'a, T>(data: &'a T) -> Container<'a, T> {
    todo!("Implement make_container")
}

/// Compare two containers.
pub fn compare_containers<'a, T: PartialEq>(
    c1: &Container<'a, T>,
    c2: &Container<'a, T>,
) -> bool  {
    todo!("Compare two containers.")
}

pub struct Pair<'a, T, U> {
    first: &'a T,
    second: &'a U,
}

impl<'a, T, U> Pair<'a, T, U> {
    pub fn new(first: &'a T, second: &'a U) -> Self {
        todo!("Implement new")
    }
    
    pub fn first(&self) -> &T {
        todo!("Implement first")
    }
    
    pub fn second(&self) -> &U {
        todo!("Implement second")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_new() {
        let value = 42;
        let container = Container::new(&value);
        assert_eq!(*container.get(), 42);
    }

    #[test]
    fn test_make_container() {
        let value = String::from("test");
        let container = make_container(&value);
        assert_eq!(container.get(), "test");
    }

    #[test]
    fn test_compare_containers() {
        let val1 = 10;
        let val2 = 10;
        let val3 = 20;
        
        let c1 = Container::new(&val1);
        let c2 = Container::new(&val2);
        let c3 = Container::new(&val3);
        
        assert!(compare_containers(&c1, &c2));
        assert!(!compare_containers(&c1, &c3));
    }

    #[test]
    fn test_pair() {
        let x = 42;
        let y = String::from("hello");
        let pair = Pair::new(&x, &y);
        
        assert_eq!(*pair.first(), 42);
        assert_eq!(pair.second(), "hello");
    }

    #[test]
    fn test_container_lifetime() {
        let container;
        {
            let value = 100;
            container = Container::new(&value);
            assert_eq!(*container.get(), 100);
        }
        // container cannot be used here - value is out of scope
    }
}
