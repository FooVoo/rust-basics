//! Exercise 05: Chaining Operations - Combine multiple operations
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Chain map, filter, and other operations
//! - Understand lazy evaluation
//! - Write fluent iterator code

/// Filter even numbers and double them.
pub fn filter_and_double_evens(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * 2)
        .collect()
}

/// Get lengths of strings longer than min_length, then sum them.
pub fn sum_long_string_lengths(strings: &[&str], min_length: usize) -> usize {
    strings
        .iter()
        .filter(|s| s.len() > min_length)
        .map(|s| s.len())
        .sum()
}

/// Square positive numbers and collect them.
pub fn square_positive(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&n| n > 0)
        .map(|&n| n * n)
        .collect()
}

/// Convert strings to uppercase, filter those starting with 'A', and collect.
pub fn uppercase_starting_with_a(strings: &[&str]) -> Vec<String> {
    strings
        .iter()
        .map(|s| s.to_uppercase())
        .filter(|s| s.starts_with('A'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_and_double_evens() {
        assert_eq!(filter_and_double_evens(&[1, 2, 3, 4, 5]), vec![4, 8]);
        assert_eq!(filter_and_double_evens(&[]), vec![]);
        assert_eq!(filter_and_double_evens(&[1, 3, 5]), vec![]);
        assert_eq!(filter_and_double_evens(&[2, 4, 6]), vec![4, 8, 12]);
    }

    #[test]
    fn test_sum_long_string_lengths() {
        assert_eq!(sum_long_string_lengths(&["a", "bb", "ccc", "dddd"], 2), 7);
        assert_eq!(sum_long_string_lengths(&[], 0), 0);
        assert_eq!(sum_long_string_lengths(&["hi", "yo"], 5), 0);
        assert_eq!(sum_long_string_lengths(&["hello", "world"], 3), 10);
    }

    #[test]
    fn test_square_positive() {
        assert_eq!(square_positive(&[1, -2, 3, -4, 5]), vec![1, 9, 25]);
        assert_eq!(square_positive(&[]), vec![]);
        assert_eq!(square_positive(&[-1, -2, -3]), vec![]);
        assert_eq!(square_positive(&[2, 3, 4]), vec![4, 9, 16]);
    }

    #[test]
    fn test_uppercase_starting_with_a() {
        assert_eq!(
            uppercase_starting_with_a(&["apple", "banana", "avocado"]),
            vec!["APPLE", "AVOCADO"]
        );
        assert_eq!(uppercase_starting_with_a(&[]), Vec::<String>::new());
        assert_eq!(
            uppercase_starting_with_a(&["ant", "ant", "bee"]),
            vec!["ANT", "ANT"]
        );
    }
}
