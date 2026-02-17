//! Exercise 04: Option::filter - Conditional filtering
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Option::filter combinator
//! - Apply predicates to Option values
//! - Convert Some to None based on conditions

/// Keep only even numbers, return None for odd numbers.
pub fn keep_even(value: Option<i32>) -> Option<i32> {
    value.filter(|&x| x % 2 == 0)
}

/// Keep only positive numbers.
pub fn keep_positive(value: Option<i32>) -> Option<i32> {
    value.filter(|&x| x > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_even() {
        assert_eq!(keep_even(Some(4)), Some(4));
        assert_eq!(keep_even(Some(7)), None);
        assert_eq!(keep_even(Some(0)), Some(0));
        assert_eq!(keep_even(None), None);
    }

    #[test]
    fn test_keep_positive() {
        assert_eq!(keep_positive(Some(5)), Some(5));
        assert_eq!(keep_positive(Some(-3)), None);
        assert_eq!(keep_positive(Some(0)), None);
        assert_eq!(keep_positive(None), None);
    }
}
