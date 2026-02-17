//! Exercise 27: Option and Result with iterators
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Combine iterators with Option and Result
//! - Use filter_map, flat_map with Options
//! - Handle collections of optional values

/// Find the first even number in a slice.
pub fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

/// Parse and sum all valid numbers.
pub fn sum_valid_numbers(strings: &[&str]) -> i32 {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum()
}

/// Find the maximum of parsed numbers.
pub fn max_parsed(strings: &[&str]) -> Option<i32> {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .max()
}

/// Chain multiple optional iterators.
pub fn chain_options(
    first: Option<Vec<i32>>,
    second: Option<Vec<i32>>,
) -> Vec<i32> {
    first
        .into_iter()
        .flatten()
        .chain(second.into_iter().flatten())
        .collect()
}

/// Map and filter in one pass.
pub fn process_strings(strings: &[&str]) -> Vec<i32> {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|&n| n > 0)
        .map(|n| n * 2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_even() {
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
        assert_eq!(find_first_even(&[1, 3, 5]), None);
        assert_eq!(find_first_even(&[2, 4, 6]), Some(2));
    }

    #[test]
    fn test_sum_valid_numbers() {
        assert_eq!(sum_valid_numbers(&["1", "2", "abc", "3"]), 6);
        assert_eq!(sum_valid_numbers(&["abc", "xyz"]), 0);
    }

    #[test]
    fn test_max_parsed() {
        assert_eq!(max_parsed(&["1", "5", "3", "abc"]), Some(5));
        assert_eq!(max_parsed(&["abc"]), None);
    }

    #[test]
    fn test_chain_options() {
        assert_eq!(
            chain_options(Some(vec![1, 2]), Some(vec![3, 4])),
            vec![1, 2, 3, 4]
        );
        assert_eq!(chain_options(None, Some(vec![1, 2])), vec![1, 2]);
        assert_eq!(chain_options(Some(vec![1, 2]), None), vec![1, 2]);
        assert_eq!(chain_options(None, None), vec![]);
    }

    #[test]
    fn test_process_strings() {
        assert_eq!(
            process_strings(&["1", "2", "abc", "-5", "3"]),
            vec![2, 4, 6]
        );
        assert_eq!(process_strings(&["0", "-1", "-2"]), vec![]);
    }
}
