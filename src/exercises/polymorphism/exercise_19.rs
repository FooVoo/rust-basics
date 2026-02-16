//! Exercise 19: Conditional Trait Implementation - Implement traits conditionally
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement traits conditionally with where clauses
//! - Understand blanket implementations with constraints
//! - Use trait bounds in impl blocks

use std::fmt::Display;

pub trait Printable {
    fn print(&self) -> String;
}

// Blanket implementation for types that implement Display
impl<T: Display> Printable for T {
    fn print(&self) -> String {
        format!("Printing: {}", self)
    }
}

pub trait Container<T> {
    fn get(&self) -> &T;
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

// Conditional implementation: Wrapper<T> gets PrettyPrint if T: Display
impl<T: Display> PrettyPrint for Wrapper<T> {
    fn pretty_print(&self) -> String {
        format!("Container[{}]", self.get())
    }
}

pub struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

impl<T> Container<T> for Wrapper<T> {
    fn get(&self) -> &T {
        &self.value
    }
}

pub trait Summable {
    type Item;
    fn sum(&self) -> Self::Item;
}

// Implement Summable for Vec<T> only when T can be added and has a default
impl<T> Summable for Vec<T>
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    type Item = T;
    
    fn sum(&self) -> Self::Item {
        self.iter().fold(T::default(), |acc, &x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printable_for_i32() {
        let num = 42;
        assert_eq!(num.print(), "Printing: 42");
    }

    #[test]
    fn test_printable_for_string() {
        let text = "hello".to_string();
        assert_eq!(text.print(), "Printing: hello");
    }

    #[test]
    fn test_wrapper_container() {
        let wrapper = Wrapper::new(100);
        assert_eq!(*wrapper.get(), 100);
    }

    #[test]
    fn test_pretty_print_with_display() {
        let wrapper = Wrapper::new(42);
        // wrapper implements Container<i32> and i32 implements Display
        // so wrapper gets PrettyPrint automatically
        assert_eq!(wrapper.pretty_print(), "Container[42]");
    }

    #[test]
    fn test_pretty_print_with_string() {
        let wrapper = Wrapper::new("test".to_string());
        assert_eq!(wrapper.pretty_print(), "Container[test]");
    }

    #[test]
    fn test_vec_summable_i32() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(numbers.sum(), 15);
    }

    #[test]
    fn test_vec_summable_f64() {
        let numbers = vec![1.5, 2.5, 3.0];
        assert_eq!(numbers.sum(), 7.0);
    }

    #[test]
    fn test_vec_summable_empty() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(numbers.sum(), 0);
    }

    #[test]
    fn test_blanket_implementation() {
        // Any type implementing Display gets Printable automatically
        assert_eq!(123.print(), "Printing: 123");
        assert_eq!(true.print(), "Printing: true");
        assert_eq!('X'.print(), "Printing: X");
    }

    #[test]
    fn test_conditional_trait_bounds() {
        let int_wrapper = Wrapper::new(999);
        let str_wrapper = Wrapper::new("data");
        
        // Both can use pretty_print because their inner types implement Display
        assert!(int_wrapper.pretty_print().contains("999"));
        assert!(str_wrapper.pretty_print().contains("data"));
    }

    #[test]
    fn test_multiple_blanket_impls() {
        let value = 42;
        
        // Gets Printable from blanket impl
        let printed = value.print();
        assert!(printed.contains("42"));
        
        // Can also use Display directly
        assert_eq!(format!("{}", value), "42");
    }
}
