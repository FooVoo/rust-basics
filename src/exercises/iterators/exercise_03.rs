//! Exercise 03: Filter Operations - Select specific elements
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use filter() to select elements
//! - Apply multiple filter conditions
//! - Combine filter with other operations

/// Filter even numbers from a slice.
pub fn filter_even(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&n| n % 2 == 0).copied().collect()
}

/// Filter strings longer than a given length.
pub fn filter_long_strings(strings: &[&str], min_length: usize) -> Vec<String> {
    strings
        .iter()
        .filter(|s| s.len() > min_length)
        .map(|&s| s.to_string())
        .collect()
}

/// Filter numbers in a specific range [min, max].
pub fn filter_in_range(numbers: &[i32], min: i32, max: i32) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&n| n >= min && n <= max)
        .copied()
        .collect()
}

/// Filter out empty strings.
pub fn filter_non_empty(strings: &[&str]) -> Vec<String> {
    strings
        .iter()
        .filter(|s| !s.is_empty())
        .map(|&s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_even() {
        assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(filter_even(&[]), vec![]);
        assert_eq!(filter_even(&[1, 3, 5]), vec![]);
        assert_eq!(filter_even(&[0, 2, 4]), vec![0, 2, 4]);
    }

    #[test]
    fn test_filter_long_strings() {
        assert_eq!(
            filter_long_strings(&["a", "bb", "ccc", "dddd"], 2),
            vec!["ccc", "dddd"]
        );
        assert_eq!(filter_long_strings(&[], 0), Vec::<String>::new());
        assert_eq!(
            filter_long_strings(&["hello", "hi", "world"], 3),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn test_filter_in_range() {
        assert_eq!(filter_in_range(&[1, 5, 10, 15, 20], 5, 15), vec![5, 10, 15]);
        assert_eq!(filter_in_range(&[], 0, 10), vec![]);
        assert_eq!(filter_in_range(&[1, 2, 3], 10, 20), vec![]);
        assert_eq!(filter_in_range(&[5, 5, 5], 5, 5), vec![5, 5, 5]);
    }

    #[test]
    fn test_filter_non_empty() {
        assert_eq!(
            filter_non_empty(&["a", "", "b", "", "c"]),
            vec!["a", "b", "c"]
        );
        assert_eq!(filter_non_empty(&[]), Vec::<String>::new());
        assert_eq!(filter_non_empty(&["", "", ""]), Vec::<String>::new());
        assert_eq!(filter_non_empty(&["hello", "world"]), vec!["hello", "world"]);
    }
}
