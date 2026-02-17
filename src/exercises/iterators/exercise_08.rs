//! Exercise 08: Take and Skip - Control iteration flow
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use take() to limit iterations
//! - Use skip() to skip elements
//! - Combine take/skip with other operations

/// Get the first n elements.
pub fn take_first(numbers: &[i32], n: usize) -> Vec<i32>  {
    todo!("Get the first n elements.")
}

/// Skip the first n elements and collect the rest.
pub fn skip_first(numbers: &[i32], n: usize) -> Vec<i32>  {
    todo!("Skip the first n elements and collect the rest.")
}

/// Take while condition is true.
pub fn take_while_positive(numbers: &[i32]) -> Vec<i32>  {
    todo!("Take while condition is true.")
}

/// Skip first n, then take next m elements.
pub fn skip_and_take(numbers: &[i32], skip_count: usize, take_count: usize) -> Vec<i32>  {
    todo!("Skip first n, then take next m elements.")
}

/// Take every nth element.
pub fn take_every_nth(numbers: &[i32], n: usize) -> Vec<i32>  {
    todo!("Take every nth element.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_first() {
        assert_eq!(take_first(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
        assert_eq!(take_first(&[1, 2], 5), vec![1, 2]);
        assert_eq!(take_first(&[], 3), vec![]);
        assert_eq!(take_first(&[1, 2, 3], 0), vec![]);
    }

    #[test]
    fn test_skip_first() {
        assert_eq!(skip_first(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
        assert_eq!(skip_first(&[1, 2], 5), vec![]);
        assert_eq!(skip_first(&[], 3), vec![]);
        assert_eq!(skip_first(&[1, 2, 3], 0), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_while_positive() {
        assert_eq!(take_while_positive(&[1, 2, 3, -1, 4]), vec![1, 2, 3]);
        assert_eq!(take_while_positive(&[-1, 2, 3]), vec![]);
        assert_eq!(take_while_positive(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(take_while_positive(&[]), vec![]);
    }

    #[test]
    fn test_skip_and_take() {
        assert_eq!(skip_and_take(&[1, 2, 3, 4, 5], 1, 3), vec![2, 3, 4]);
        assert_eq!(skip_and_take(&[1, 2], 5, 2), vec![]);
        assert_eq!(skip_and_take(&[1, 2, 3, 4], 2, 1), vec![3]);
    }

    #[test]
    fn test_take_every_nth() {
        assert_eq!(take_every_nth(&[1, 2, 3, 4, 5, 6], 2), vec![1, 3, 5]);
        assert_eq!(take_every_nth(&[1, 2, 3], 1), vec![1, 2, 3]);
        assert_eq!(take_every_nth(&[], 2), vec![]);
        assert_eq!(take_every_nth(&[1, 2, 3, 4, 5], 0), vec![]);
    }
}
