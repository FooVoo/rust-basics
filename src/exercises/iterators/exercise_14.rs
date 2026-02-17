//! Exercise 14: Filter Map - Combined filter and map
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use filter_map() efficiently
//! - Convert and filter in one operation
//! - Handle Option results in iterations

/// Parse strings to numbers, ignoring invalid ones.
pub fn parse_numbers(strings: &[&str]) -> Vec<i32>  {
    todo!("Parse strings to numbers, ignoring invalid ones.")
}

/// Get lengths of non-empty strings.
pub fn non_empty_lengths(strings: &[&str]) -> Vec<usize>  {
    todo!("Get lengths of non-empty strings.")
}

/// Square positive numbers only.
pub fn square_positives(numbers: &[i32]) -> Vec<i32>  {
    todo!("Square positive numbers only.")
}

/// Extract first character of non-empty strings.
pub fn first_chars(strings: &[&str]) -> Vec<char>  {
    todo!("Extract first character of non-empty strings.")
}

/// Divide numbers, skipping zeros in divisor.
pub fn safe_divisions(dividends: &[i32], divisors: &[i32]) -> Vec<i32>  {
    todo!("Divide numbers, skipping zeros in divisor.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers(&["1", "2", "abc", "3"]), vec![1, 2, 3]);
        assert_eq!(parse_numbers(&["abc", "def"]), vec![]);
        assert_eq!(parse_numbers(&[]), vec![]);
        assert_eq!(parse_numbers(&["42", "-10"]), vec![42, -10]);
    }

    #[test]
    fn test_non_empty_lengths() {
        assert_eq!(non_empty_lengths(&["a", "", "bb", ""]), vec![1, 2]);
        assert_eq!(non_empty_lengths(&["", ""]), vec![]);
        assert_eq!(non_empty_lengths(&["hello", "world"]), vec![5, 5]);
    }

    #[test]
    fn test_square_positives() {
        assert_eq!(square_positives(&[1, -2, 3, -4, 5]), vec![1, 9, 25]);
        assert_eq!(square_positives(&[-1, -2, -3]), vec![]);
        assert_eq!(square_positives(&[]), vec![]);
        assert_eq!(square_positives(&[2, 3]), vec![4, 9]);
    }

    #[test]
    fn test_first_chars() {
        assert_eq!(first_chars(&["apple", "banana", ""]), vec!['a', 'b']);
        assert_eq!(first_chars(&["", "", ""]), vec![]);
        assert_eq!(first_chars(&["x"]), vec!['x']);
    }

    #[test]
    fn test_safe_divisions() {
        assert_eq!(safe_divisions(&[10, 20, 30], &[2, 0, 5]), vec![5, 6]);
        assert_eq!(safe_divisions(&[10], &[0]), vec![]);
        assert_eq!(safe_divisions(&[], &[]), vec![]);
        assert_eq!(safe_divisions(&[10, 20], &[2, 4]), vec![5, 5]);
    }
}
