//! Exercise 06: Collect Variations - Different collection types
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Collect into different collection types
//! - Use collect() with type inference
//! - Work with HashSet and HashMap

use std::collections::{HashMap, HashSet};

/// Collect unique numbers into a HashSet.
pub fn unique_numbers(numbers: &[i32]) -> HashSet<i32> {
    todo!("Implement unique_numbers")
}

/// Create a frequency map of characters.
pub fn char_frequency(s: &str) -> HashMap<char, usize> {
    todo!("Implement char_frequency")
}

/// Collect words and their lengths into a HashMap.
pub fn word_lengths(words: &[&str]) -> HashMap<String, usize> {
    todo!("Implement word_lengths")
}

/// Partition numbers into even and odd groups.
pub fn partition_even_odd(numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    todo!("Implement partition_even_odd")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_numbers() {
        let result = unique_numbers(&[1, 2, 2, 3, 3, 3]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));

        let result = unique_numbers(&[]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_char_frequency() {
        let result = char_frequency("hello");
        assert_eq!(result.get(&'h'), Some(&1));
        assert_eq!(result.get(&'e'), Some(&1));
        assert_eq!(result.get(&'l'), Some(&2));
        assert_eq!(result.get(&'o'), Some(&1));

        let result = char_frequency("");
        assert_eq!(result.len(), 0);

        let result = char_frequency("aaa");
        assert_eq!(result.get(&'a'), Some(&3));
    }

    #[test]
    fn test_word_lengths() {
        let result = word_lengths(&["hi", "hello", "world"]);
        assert_eq!(result.get("hi"), Some(&2));
        assert_eq!(result.get("hello"), Some(&5));
        assert_eq!(result.get("world"), Some(&5));

        let result = word_lengths(&[]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_partition_even_odd() {
        let (evens, odds) = partition_even_odd(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);

        let (evens, odds) = partition_even_odd(&[]);
        assert_eq!(evens, Vec::<i32>::new());
        assert_eq!(odds, Vec::<i32>::new());

        let (evens, odds) = partition_even_odd(&[2, 4, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, Vec::<i32>::new());
    }
}
