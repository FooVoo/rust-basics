//! Exercise 08: Option::unwrap_or_else - Lazy default computation
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand unwrap_or_else for lazy evaluation
//! - Avoid unnecessary computation
//! - Use closures for default values

/// Get value or compute default lazily.
pub fn get_or_compute<F>(value: Option<i32>, compute: F) -> i32
where
    F: FnOnce() -> i32,
 {
    todo!("Get value or compute default lazily.")
}

/// Get first even number or compute sum of all.
pub fn first_even_or_sum(numbers: &[i32]) -> i32 {
    todo!("Implement first_even_or_sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_or_compute() {
        assert_eq!(get_or_compute(Some(10), || 5), 10);
        assert_eq!(get_or_compute(None, || 5), 5);
        assert_eq!(get_or_compute(None, || 2 + 3), 5);
    }

    #[test]
    fn test_first_even_or_sum() {
        assert_eq!(first_even_or_sum(&[1, 3, 4, 5]), 4);
        assert_eq!(first_even_or_sum(&[1, 3, 5]), 9);
        assert_eq!(first_even_or_sum(&[2, 4, 6]), 2);
        assert_eq!(first_even_or_sum(&[]), 0);
    }
}
