//! Exercise 14: Option::zip - Combine two Options
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Combine multiple Options
//! - Work with tuples
//! - Handle paired operations

/// Add two optional values.
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.zip(b).map(|(x, y)| x + y)
}

/// Multiply three optional values.
pub fn multiply_three(a: Option<i32>, b: Option<i32>, c: Option<i32>) -> Option<i32> {
    a.zip(b)
        .zip(c)
        .map(|((x, y), z)| x * y * z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_options() {
        assert_eq!(add_options(Some(5), Some(10)), Some(15));
        assert_eq!(add_options(Some(3), None), None);
        assert_eq!(add_options(None, Some(7)), None);
        assert_eq!(add_options(None, None), None);
    }

    #[test]
    fn test_multiply_three() {
        assert_eq!(multiply_three(Some(2), Some(3), Some(4)), Some(24));
        assert_eq!(multiply_three(Some(2), None, Some(4)), None);
        assert_eq!(multiply_three(None, None, None), None);
    }
}
