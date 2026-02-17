//! Exercise 08: Generic Math - Use numeric trait bounds
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use multiple trait bounds
//! - Work with numeric operations
//! - Combine traits with + syntax

use std::ops::Add;

/// Adds two values together.
pub fn add<T: Add<Output = T>>(a: T, b: T) -> T  {
    todo!("Add two values together.")
}

/// Sums all elements in a slice.
pub fn sum_all<T>(slice: &[T]) -> T
where
    T: Add<Output = T> + Copy + Default,
 {
    todo!("Sums all elements in a slice.")
}

/// A generic accumulator for addable types.
pub struct Accumulator<T>
where
    T: Add<Output = T> + Copy,
{
    total: T,
}

impl<T> Accumulator<T>
where
    T: Add<Output = T> + Copy,
{
    /// Creates a new Accumulator with an initial value.
    pub fn new(initial: T) -> Self  {
        todo!("Create a new Accumulator with an initial value.")
    }

    /// Adds a value to the accumulator.
    pub fn add(&mut self, value: T)  {
        todo!("Add a value to the accumulator.")
    }

    /// Returns the current total.
    pub fn total(&self) -> T  {
        todo!("Return the current total.")
    }

    /// Resets to a new value.
    pub fn reset(&mut self, value: T)  {
        todo!("Resets to a new value.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_integers() {
        assert_eq!(add(5, 10), 15);
    }

    #[test]
    fn test_add_floats() {
        assert_eq!(add(3.5, 2.5), 6.0);
    }

    #[test]
    fn test_add_strings() {
        // Note: String doesn't implement Add<String>, only Add<&str>
        // This test is removed as the generic add function requires Add<Output = T>
        // For strings, use format! or push_str instead
    }

    #[test]
    fn test_sum_all_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_all(&numbers), 15);
    }

    #[test]
    fn test_sum_all_empty() {
        let numbers: Vec<i32> = vec![];
        assert_eq!(sum_all(&numbers), 0);
    }

    #[test]
    fn test_sum_all_floats() {
        let numbers = vec![1.5, 2.5, 3.0];
        assert_eq!(sum_all(&numbers), 7.0);
    }

    #[test]
    fn test_accumulator() {
        let mut acc = Accumulator::new(0);
        acc.add(5);
        acc.add(10);
        assert_eq!(acc.total(), 15);
    }

    #[test]
    fn test_accumulator_floats() {
        let mut acc = Accumulator::new(0.0);
        acc.add(1.5);
        acc.add(2.5);
        assert_eq!(acc.total(), 4.0);
    }

    #[test]
    fn test_accumulator_reset() {
        let mut acc = Accumulator::new(100);
        acc.add(50);
        acc.reset(0);
        assert_eq!(acc.total(), 0);
    }
}
