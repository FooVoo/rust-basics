//! Exercise 15: While Let - Iterating with Pattern Matching
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use while let for conditional iteration
//! - Process sequences with pattern matching
//! - Understand while let vs match

/// Processes a stack of options, returning sum of all Some values
pub fn sum_stack(mut stack: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    while let Some(item) = stack.pop() {
        if let Some(value) = item {
            sum += value;
        }
    }
    sum
}

/// Counts consecutive Some values from the start of the vector
pub fn count_consecutive_some(values: &[Option<i32>]) -> usize {
    let mut count = 0;
    let mut iter = values.iter();
    while let Some(Some(_)) = iter.next() {
        count += 1;
    }
    count
}

/// Extracts all Some values from a vector
pub fn extract_some_values(values: Vec<Option<i32>>) -> Vec<i32> {
    values.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_stack() {
        assert_eq!(sum_stack(vec![Some(1), Some(2), Some(3)]), 6);
        assert_eq!(sum_stack(vec![Some(1), None, Some(3)]), 4);
        assert_eq!(sum_stack(vec![None, None]), 0);
        assert_eq!(sum_stack(vec![]), 0);
    }

    #[test]
    fn test_count_consecutive_some() {
        assert_eq!(count_consecutive_some(&[Some(1), Some(2), Some(3)]), 3);
        assert_eq!(count_consecutive_some(&[Some(1), Some(2), None, Some(3)]), 2);
        assert_eq!(count_consecutive_some(&[None, Some(1)]), 0);
        assert_eq!(count_consecutive_some(&[]), 0);
    }

    #[test]
    fn test_extract_some_values() {
        assert_eq!(extract_some_values(vec![Some(1), Some(2), Some(3)]), vec![1, 2, 3]);
        assert_eq!(extract_some_values(vec![Some(1), None, Some(3)]), vec![1, 3]);
        assert_eq!(extract_some_values(vec![None, None]), vec![]);
    }
}
