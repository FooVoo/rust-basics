//! Exercise 05: Array Access Safety - Safe array indexing
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Handle array bounds checking
//! - Convert Option to Result
//! - Provide descriptive error messages for out-of-bounds access

/// Safely get an element from a slice at the given index.
/// Return Ok(element) if index is valid, Err with message otherwise.
pub fn safe_get<T: Clone>(slice: &[T], index: usize) -> Result<T, String> {
    todo!("Implement safe_get")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_get_success() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(safe_get(&arr, 0), Ok(1));
        assert_eq!(safe_get(&arr, 2), Ok(3));
        assert_eq!(safe_get(&arr, 4), Ok(5));
    }

    #[test]
    fn test_safe_get_out_of_bounds() {
        let arr = vec![1, 2, 3];
        assert!(safe_get(&arr, 3).is_err());
        assert!(safe_get(&arr, 10).is_err());
        assert_eq!(
            safe_get(&arr, 5),
            Err("Index 5 out of bounds for slice of length 3".to_string())
        );
    }

    #[test]
    fn test_safe_get_empty_slice() {
        let arr: Vec<i32> = vec![];
        assert!(safe_get(&arr, 0).is_err());
    }

    #[test]
    fn test_safe_get_strings() {
        let strings = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(safe_get(&strings, 0), Ok("hello".to_string()));
        assert!(safe_get(&strings, 2).is_err());
    }
}
