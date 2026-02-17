//! Exercise 03: Multiple Trait Implementations - Implement multiple traits for one type
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Implement multiple traits for a single type
//! - Understand trait composition
//! - Use multiple trait bounds

pub trait Named {
    fn name(&self) -> &str;
}

pub trait Aged {
    fn age(&self) -> u32;
}

pub trait Greetable {
    fn greet(&self) -> String;
}

pub struct Student {
    pub name: String,
    pub age: u32,
    pub grade: String,
}

impl Named for Student {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Aged for Student {
    fn age(&self) -> u32 {
        self.age
    }
}

impl Greetable for Student {
    fn greet(&self) -> String {
        format!("Hi, I'm {} and I'm in grade {}", self.name, self.grade)
    }
}

/// A function that works with anything that is both Named and Aged.
pub fn introduce<T: Named + Aged>(entity: &T) -> String {
    format!("{} is {} years old", entity.name(), entity.age())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_named() {
        let student = Student {
            name: "Alice".to_string(),
            age: 16,
            grade: "10th".to_string(),
        };
        assert_eq!(student.name(), "Alice");
    }

    #[test]
    fn test_student_aged() {
        let student = Student {
            name: "Bob".to_string(),
            age: 17,
            grade: "11th".to_string(),
        };
        assert_eq!(student.age(), 17);
    }

    #[test]
    fn test_student_greetable() {
        let student = Student {
            name: "Charlie".to_string(),
            age: 15,
            grade: "9th".to_string(),
        };
        assert_eq!(student.greet(), "Hi, I'm Charlie and I'm in grade 9th");
    }

    #[test]
    fn test_introduce_function() {
        let student = Student {
            name: "Diana".to_string(),
            age: 18,
            grade: "12th".to_string(),
        };
        assert_eq!(introduce(&student), "Diana is 18 years old");
    }

    #[test]
    fn test_all_traits() {
        let student = Student {
            name: "Eve".to_string(),
            age: 16,
            grade: "10th".to_string(),
        };
        
        assert_eq!(student.name(), "Eve");
        assert_eq!(student.age(), 16);
        assert!(student.greet().contains("Eve"));
    }
}
