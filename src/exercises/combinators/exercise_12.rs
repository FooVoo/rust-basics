//! Exercise 12: Option::ok_or - Convert Option to Result
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Convert Option to Result
//! - Provide error values for None
//! - Bridge Option and Result types

/// Find element and return as Result.
pub fn find_element(numbers: &[i32], target: i32) -> Result<usize, String> {
    todo!("Implement find_element")
}

/// Get first element as Result.
pub fn first_element<T: Clone>(slice: &[T]) -> Result<T, String> {
    todo!("Implement first_element")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_element() {
        assert_eq!(find_element(&[1, 2, 3, 4], 3), Ok(2));
        assert_eq!(find_element(&[10, 20, 30], 20), Ok(1));
        assert!(find_element(&[1, 2, 3], 5).is_err());
    }

    #[test]
    fn test_first_element() {
        assert_eq!(first_element(&[1, 2, 3]), Ok(1));
        assert_eq!(first_element(&["a", "b"]), Ok("a"));
        assert!(first_element::<i32>(&[]).is_err());
    }
}
