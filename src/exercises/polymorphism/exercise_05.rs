//! Exercise 05: Trait Implementation for Primitive Types - Implement traits for built-in types
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Implement traits for primitive types
//! - Use newtype pattern when needed
//! - Understand orphan rules

pub trait Doubling {
    fn double(&self) -> Self;
}

// Implement for a newtype wrapper around i32
pub struct MyInt(pub i32);

impl Doubling for MyInt {
    fn double(&self) -> Self {
        todo!("Implement double")
    }
}

// Implement for a newtype wrapper around String
pub struct MyString(pub String);

impl Doubling for MyString {
    fn double(&self) -> Self {
        todo!("Implement double")
    }
}

pub trait Tripling {
    fn triple(&self) -> Self;
}

impl Tripling for MyInt {
    fn triple(&self) -> Self {
        todo!("Implement triple")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myint_double() {
        let num = MyInt(5);
        let doubled = num.double();
        assert_eq!(doubled.0, 10);
    }

    #[test]
    fn test_myint_double_negative() {
        let num = MyInt(-3);
        let doubled = num.double();
        assert_eq!(doubled.0, -6);
    }

    #[test]
    fn test_mystring_double() {
        let s = MyString("hello".to_string());
        let doubled = s.double();
        assert_eq!(doubled.0, "hellohello");
    }

    #[test]
    fn test_mystring_double_empty() {
        let s = MyString("".to_string());
        let doubled = s.double();
        assert_eq!(doubled.0, "");
    }

    #[test]
    fn test_myint_triple() {
        let num = MyInt(4);
        let tripled = num.triple();
        assert_eq!(tripled.0, 12);
    }

    #[test]
    fn test_multiple_operations() {
        let num = MyInt(2);
        let doubled = num.double();
        let tripled = doubled.triple();
        assert_eq!(tripled.0, 12); // 2 * 2 * 3
    }
}
