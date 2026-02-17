//! Exercise 08: Struct Ownership - Ownership with custom types
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand ownership with structs
//! - Borrow struct fields
//! - Transfer ownership of structs

#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn get_age(&self) -> u32 {
        self.age
    }
    
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }
}

/// Create a person and return it.
pub fn create_person(name: &str, age: u32) -> Person {
    Person::new(name.to_string(), age)
}

/// Get a description of a person.
pub fn describe_person(person: &Person) -> String {
    format!("{} is {} years old", person.get_name(), person.get_age())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_person() {
        let person = create_person("Alice", 30);
        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 30);
    }

    #[test]
    fn test_person_methods() {
        let person = create_person("Bob", 25);
        assert_eq!(person.get_name(), "Bob");
        assert_eq!(person.get_age(), 25);
    }

    #[test]
    fn test_have_birthday() {
        let mut person = create_person("Charlie", 20);
        person.have_birthday();
        assert_eq!(person.age, 21);
        person.have_birthday();
        assert_eq!(person.age, 22);
    }

    #[test]
    fn test_describe_person() {
        let person = create_person("Diana", 35);
        let desc = describe_person(&person);
        assert_eq!(desc, "Diana is 35 years old");
        // person still valid after borrowing
        assert_eq!(person.age, 35);
    }

    #[test]
    fn test_clone_person() {
        let person1 = create_person("Eve", 28);
        let mut person2 = person1.clone();
        
        person2.have_birthday();
        
        assert_eq!(person1.age, 28);
        assert_eq!(person2.age, 29);
    }
}
