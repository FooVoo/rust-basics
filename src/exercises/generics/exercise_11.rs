//! Exercise 11: Where Clauses - Use where clauses for complex bounds
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use where clauses for readability
//! - Apply multiple trait bounds
//! - Understand when to use where vs inline bounds

use std::fmt::Display;

/// A function with multiple trait bounds using where clause.
pub fn print_and_clone<T>(value: T) -> T
where
    T: Display + Clone,
 {
    todo!("A function with multiple trait bounds using where clause.")
}

/// A function with complex bounds on multiple parameters.
pub fn compare_and_display<T, U>(a: T, b: U) -> String
where
    T: Display + PartialOrd,
    U: Display + Into<T>,
 {
    todo!("A function with complex bounds on multiple parameters.")
}

/// A generic struct with where clause constraints.
pub struct Processor<T, U>
where
    T: Clone,
    U: Default,
{
    input: T,
    output: U,
}

impl<T, U> Processor<T, U>
where
    T: Clone,
    U: Default,
{
    /// Creates a new Processor with default output.
    pub fn new(input: T) -> Self  {
        todo!("Create a new Processor with default output.")
    }

    /// Gets a clone of the input.
    pub fn get_input(&self) -> T  {
        todo!("Get a clone of the input.")
    }

    /// Gets a reference to the output.
    pub fn get_output(&self) -> &U  {
        todo!("Get a reference to the output.")
    }

    /// Sets the output value.
    pub fn set_output(&mut self, output: U)  {
        todo!("Set the output value.")
    }
}

impl<T, U> Processor<T, U>
where
    T: Clone + Display,
    U: Default + Display,
{
    /// Displays both input and output.
    pub fn display_both(&self) -> String  {
        todo!("Displays both input and output.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_and_clone() {
        let value = 42;
        let cloned = print_and_clone(value);
        assert_eq!(cloned, 42);
    }

    #[test]
    fn test_print_and_clone_string() {
        let value = "hello".to_string();
        let cloned = print_and_clone(value.clone());
        assert_eq!(cloned, value);
    }

    #[test]
    fn test_compare_and_display_greater() {
        let result = compare_and_display(10, 5);
        assert!(result.contains("greater than"));
    }

    #[test]
    fn test_compare_and_display_not_greater() {
        let result = compare_and_display(5, 10);
        assert!(result.contains("not greater than"));
    }

    #[test]
    fn test_processor_new() {
        let processor: Processor<i32, String> = Processor::new(42);
        assert_eq!(processor.get_input(), 42);
        assert_eq!(processor.get_output(), "");
    }

    #[test]
    fn test_processor_set_output() {
        let mut processor: Processor<String, i32> = Processor::new("input".to_string());
        processor.set_output(100);
        assert_eq!(*processor.get_output(), 100);
    }

    #[test]
    fn test_processor_display_both() {
        let processor: Processor<i32, String> = Processor::new(42);
        let display = processor.display_both();
        assert!(display.contains("42"));
    }

    #[test]
    fn test_processor_with_vec() {
        let processor: Processor<Vec<i32>, Vec<String>> = Processor::new(vec![1, 2, 3]);
        assert_eq!(processor.get_input(), vec![1, 2, 3]);
        assert_eq!(processor.get_output().len(), 0);
    }
}
