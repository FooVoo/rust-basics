//! Exercise 01: Basic Trait Definition - Define and implement a simple trait
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand trait syntax
//! - Define trait methods
//! - Implement traits for types

/// A trait representing something that can be described.
pub trait Describable {
    fn describe(&self) -> String;
}

/// A simple struct representing a book.
pub struct Book {
    pub title: String,
    pub author: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

/// A simple struct representing a person.
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{}, {} years old", self.name, self.age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_describe() {
        let book = Book {
            title: "The Rust Book".to_string(),
            author: "Steve Klabnik".to_string(),
        };
        assert_eq!(book.describe(), "The Rust Book by Steve Klabnik");
    }

    #[test]
    fn test_person_describe() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        assert_eq!(person.describe(), "Alice, 30 years old");
    }

    #[test]
    fn test_different_implementations() {
        let book = Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
        };
        let person = Person {
            name: "Bob".to_string(),
            age: 25,
        };
        
        assert_ne!(book.describe(), person.describe());
    }
}
