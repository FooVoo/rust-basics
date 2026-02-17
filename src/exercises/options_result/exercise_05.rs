//! Exercise 05: Option unwrap_or_else
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use unwrap_or_else() with lazy evaluation
//! - Understand when to use unwrap_or vs unwrap_or_else
//! - Practice with closures

/// Get a value from Option or compute it lazily.
pub fn get_or_compute(opt: Option<i32>, compute: impl FnOnce() -> i32) -> i32 {
    opt.unwrap_or_else(compute)
}

/// Get the length of an optional string, or compute default length.
pub fn length_or_default(opt: Option<String>) -> usize {
    opt.unwrap_or_else(|| String::from("default")).len()
}

/// Find a value in a slice, or return the sum of all elements.
pub fn find_or_sum(numbers: &[i32], target: i32) -> i32 {
    numbers
        .iter()
        .find(|&&x| x == target)
        .copied()
        .unwrap_or_else(|| numbers.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_compute() {
        assert_eq!(get_or_compute(Some(42), || 0), 42);
        assert_eq!(get_or_compute(None, || 100), 100);
        assert_eq!(get_or_compute(None, || 1 + 1), 2);
    }

    #[test]
    fn test_length_or_default() {
        assert_eq!(length_or_default(Some(String::from("hello"))), 5);
        assert_eq!(length_or_default(None), 7); // "default" has 7 chars
    }

    #[test]
    fn test_find_or_sum() {
        assert_eq!(find_or_sum(&[1, 2, 3, 4], 3), 3);
        assert_eq!(find_or_sum(&[1, 2, 3, 4], 10), 10); // sum is 10
        assert_eq!(find_or_sum(&[5], 5), 5);
        assert_eq!(find_or_sum(&[], 1), 0); // empty sum is 0
    }
}
