//! Exercise 01: Basic Iteration - Sum of numbers
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand basic iteration with for loops
//! - Use iter() method on collections
//! - Work with iterator sum()

/// Sum all numbers in a vector using iteration.
pub fn sum_numbers(numbers: &[i32]) -> i32 {
    todo!("Implement sum_numbers")
}

/// Count how many numbers are positive.
pub fn count_positive(numbers: &[i32]) -> usize {
    todo!("Implement count_positive")
}

/// Find the maximum value in a slice, returning None if empty.
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    todo!("Implement find_max")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers() {
        assert_eq!(sum_numbers(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_numbers(&[]), 0);
        assert_eq!(sum_numbers(&[-1, -2, -3]), -6);
        assert_eq!(sum_numbers(&[10]), 10);
    }

    #[test]
    fn test_count_positive() {
        assert_eq!(count_positive(&[1, -2, 3, -4, 5]), 3);
        assert_eq!(count_positive(&[]), 0);
        assert_eq!(count_positive(&[-1, -2, -3]), 0);
        assert_eq!(count_positive(&[1, 2, 3]), 3);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 5, 3, 2, 4]), Some(5));
        assert_eq!(find_max(&[]), None);
        assert_eq!(find_max(&[-5, -1, -10]), Some(-1));
        assert_eq!(find_max(&[42]), Some(42));
    }
}
