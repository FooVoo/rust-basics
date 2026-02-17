//! Exercise 20: Iterator Consumers - Different ways to consume
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use various consuming methods
//! - Understand when iterators are consumed
//! - Choose appropriate consumption methods

/// Find if predicate matches at least n elements.
pub fn at_least_n_match(numbers: &[i32], predicate: impl Fn(i32) -> bool, n: usize) -> bool  {
    todo!("Find if predicate matches at least n elements.")
}

/// Get product of numbers, stopping early if zero found.
pub fn product_until_zero(numbers: &[i32]) -> i32  {
    todo!("Get product of numbers, stopping early if zero found.")
}

/// Check if slice is sorted.
pub fn is_sorted(numbers: &[i32]) -> bool  {
    todo!("Check if slice is sorted.")
}

/// Count elements between two values (exclusive).
pub fn count_between(numbers: &[i32], low: i32, high: i32) -> usize  {
    todo!("Count elements between two values (exclusive).")
}

/// Get last n elements.
pub fn last_n(numbers: &[i32], n: usize) -> Vec<i32>  {
    todo!("Get last n elements.")
}

/// Find the nth element that matches predicate.
pub fn nth_match(numbers: &[i32], predicate: impl Fn(i32) -> bool, n: usize) -> Option<i32>  {
    todo!("Find the nth element that matches predicate.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_least_n_match() {
        assert!(at_least_n_match(&[1, 2, 3, 4, 5], |n| n > 2, 3));
        assert!(!at_least_n_match(&[1, 2, 3], |n| n > 5, 1));
        assert!(at_least_n_match(&[], |_| true, 0));
        assert!(!at_least_n_match(&[1, 2], |n| n > 0, 3));
    }

    #[test]
    fn test_product_until_zero() {
        assert_eq!(product_until_zero(&[2, 3, 4, 0, 5]), 24);
        assert_eq!(product_until_zero(&[0, 1, 2]), 1);
        assert_eq!(product_until_zero(&[2, 3, 4]), 24);
        assert_eq!(product_until_zero(&[]), 1);
    }

    #[test]
    fn test_is_sorted() {
        assert!(is_sorted(&[1, 2, 3, 4, 5]));
        assert!(!is_sorted(&[1, 3, 2, 4]));
        assert!(is_sorted(&[]));
        assert!(is_sorted(&[1]));
        assert!(is_sorted(&[1, 1, 2, 2]));
    }

    #[test]
    fn test_count_between() {
        assert_eq!(count_between(&[1, 5, 10, 15, 20], 5, 15), 1); // only 10
        assert_eq!(count_between(&[1, 2, 3, 4, 5], 0, 3), 2); // 1, 2
        assert_eq!(count_between(&[], 0, 10), 0);
    }

    #[test]
    fn test_last_n() {
        assert_eq!(last_n(&[1, 2, 3, 4, 5], 3), vec![3, 4, 5]);
        assert_eq!(last_n(&[1, 2], 5), vec![1, 2]);
        assert_eq!(last_n(&[], 3), vec![]);
        assert_eq!(last_n(&[1, 2, 3], 0), vec![]);
    }

    #[test]
    fn test_nth_match() {
        assert_eq!(nth_match(&[1, 2, 3, 4, 5, 6], |n| n % 2 == 0, 0), Some(2));
        assert_eq!(nth_match(&[1, 2, 3, 4, 5, 6], |n| n % 2 == 0, 1), Some(4));
        assert_eq!(nth_match(&[1, 2, 3, 4, 5, 6], |n| n % 2 == 0, 2), Some(6));
        assert_eq!(nth_match(&[1, 2, 3], |n| n > 10, 0), None);
    }
}
