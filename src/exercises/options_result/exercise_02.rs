//! Exercise 02: Option unwrap and expect
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use unwrap() to extract values from Option
//! - Use expect() with custom error messages
//! - Understand when unwrap/expect are appropriate

/// Get the last element of a non-empty slice using unwrap.
/// This function assumes the slice is non-empty.
pub fn get_last_unwrap(numbers: &[i32]) -> i32  {
    todo!("This function assumes the slice is non-empty.")
}

/// Get the last element of a non-empty slice using expect.
/// This function assumes the slice is non-empty.
pub fn get_last_expect(numbers: &[i32]) -> i32  {
    todo!("This function assumes the slice is non-empty.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_last_unwrap() {
        assert_eq!(get_last_unwrap(&[1, 2, 3]), 3);
        assert_eq!(get_last_unwrap(&[42]), 42);
        assert_eq!(get_last_unwrap(&[-5, -10, -15]), -15);
    }

    #[test]
    fn test_get_last_expect() {
        assert_eq!(get_last_expect(&[1, 2, 3]), 3);
        assert_eq!(get_last_expect(&[42]), 42);
        assert_eq!(get_last_expect(&[-5, -10, -15]), -15);
    }

    #[test]
    #[should_panic]
    fn test_get_last_unwrap_panics() {
        get_last_unwrap(&[]);
    }

    #[test]
    #[should_panic(expected = "Slice should not be empty")]
    fn test_get_last_expect_panics() {
        get_last_expect(&[]);
    }
}
