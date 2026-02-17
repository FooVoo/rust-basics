//! Exercise 09: Trait Objects with Box - Use Box<dyn Trait> for dynamic dispatch
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand trait objects
//! - Use Box<dyn Trait> for heap allocation
//! - Work with heterogeneous collections

pub trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> &str;
}

pub struct Dog {
    pub name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String  {
        todo!("Implement speak")
    }
    
    fn name(&self) -> &str  {
        todo!("Implement name")
    }
}

pub struct Cat {
    pub name: String,
}

impl Animal for Cat {
    fn speak(&self) -> String  {
        todo!("Implement speak")
    }
    
    fn name(&self) -> &str  {
        todo!("Implement name")
    }
}

pub struct Bird {
    pub name: String,
}

impl Animal for Bird {
    fn speak(&self) -> String  {
        todo!("Implement speak")
    }
    
    fn name(&self) -> &str  {
        todo!("Implement name")
    }
}

/// Create a vector of different animals using trait objects
pub fn create_zoo() -> Vec<Box<dyn Animal>>  {
    todo!("Create a vector of different animals using trait objects")
}

/// Make all animals in the zoo speak
pub fn zoo_chorus(animals: &[Box<dyn Animal>]) -> Vec<String>  {
    todo!("Make all animals in the zoo speak")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog_speak() {
        let dog = Dog { name: "Buddy".to_string() };
        assert_eq!(dog.speak(), "Woof!");
    }

    #[test]
    fn test_cat_speak() {
        let cat = Cat { name: "Felix".to_string() };
        assert_eq!(cat.speak(), "Meow!");
    }

    #[test]
    fn test_trait_object() {
        let animal: Box<dyn Animal> = Box::new(Dog { name: "Max".to_string() });
        assert_eq!(animal.speak(), "Woof!");
    }

    #[test]
    fn test_create_zoo() {
        let zoo = create_zoo();
        assert_eq!(zoo.len(), 3);
    }

    #[test]
    fn test_zoo_chorus() {
        let zoo = create_zoo();
        let chorus = zoo_chorus(&zoo);
        
        assert_eq!(chorus.len(), 3);
        assert!(chorus[0].contains("Rex"));
        assert!(chorus[0].contains("Woof"));
        assert!(chorus[1].contains("Whiskers"));
        assert!(chorus[1].contains("Meow"));
        assert!(chorus[2].contains("Tweety"));
        assert!(chorus[2].contains("Tweet"));
    }

    #[test]
    fn test_heterogeneous_collection() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog { name: "A".to_string() }),
            Box::new(Cat { name: "B".to_string() }),
            Box::new(Dog { name: "C".to_string() }),
        ];
        
        assert_eq!(animals.len(), 3);
        assert_eq!(animals[0].speak(), "Woof!");
        assert_eq!(animals[1].speak(), "Meow!");
        assert_eq!(animals[2].speak(), "Woof!");
    }
}
