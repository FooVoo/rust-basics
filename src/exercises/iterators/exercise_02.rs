//! Exercise 02: Map Transformation - Transform collection elements
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use map() to transform elements
//! - Collect results into a new collection
//! - Chain iterator operations

/// Double all numbers in a vector.
pub fn double_numbers(numbers: &[i32]) -> Vec<i32> {
    todo!("Implement double_numbers")
}

/// Convert strings to their lengths.
pub fn string_lengths(strings: &[&str]) -> Vec<usize> {
    todo!("Implement string_lengths")
}

/// Square all numbers.
pub fn square_numbers(numbers: &[i32]) -> Vec<i32> {
    todo!("Implement square_numbers")
}

/// Convert numbers to strings.
pub fn numbers_to_strings(numbers: &[i32]) -> Vec<String> {
    todo!("Implement numbers_to_strings")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_numbers() {
        assert_eq!(double_numbers(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(double_numbers(&[]), vec![]);
        assert_eq!(double_numbers(&[-1, 0, 1]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_string_lengths() {
        assert_eq!(string_lengths(&["a", "bb", "ccc"]), vec![1, 2, 3]);
        assert_eq!(string_lengths(&[]), vec![]);
        assert_eq!(string_lengths(&["hello", "world"]), vec![5, 5]);
    }

    #[test]
    fn test_square_numbers() {
        assert_eq!(square_numbers(&[1, 2, 3, 4]), vec![1, 4, 9, 16]);
        assert_eq!(square_numbers(&[]), vec![]);
        assert_eq!(square_numbers(&[-2, 0, 2]), vec![4, 0, 4]);
    }

    #[test]
    fn test_numbers_to_strings() {
        assert_eq!(
            numbers_to_strings(&[1, 2, 3]),
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
        assert_eq!(numbers_to_strings(&[]), Vec::<String>::new());
    }
}
