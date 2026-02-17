//! Exercise 12: Chain and Cycle - Combining iterators
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use chain() to combine iterators
//! - Use cycle() for repeating patterns
//! - Work with multiple iterator sources

/// Concatenate multiple slices.
pub fn concat_slices(a: &[i32], b: &[i32], c: &[i32]) -> Vec<i32>  {
    todo!("Concatenate multiple slices.")
}

/// Alternate elements from two slices.
pub fn interleave(a: &[i32], b: &[i32]) -> Vec<i32>  {
    todo!("Alternate elements from two slices.")
}

/// Repeat a pattern n times.
pub fn repeat_pattern(pattern: &[i32], times: usize) -> Vec<i32>  {
    todo!("Repeat a pattern n times.")
}

/// Chain filtered results from multiple sources.
pub fn chain_filtered(a: &[i32], b: &[i32]) -> Vec<i32>  {
    todo!("Chain filtered results from multiple sources.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_slices() {
        assert_eq!(
            concat_slices(&[1, 2], &[3, 4], &[5, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(concat_slices(&[], &[1], &[2]), vec![1, 2]);
        assert_eq!(concat_slices(&[], &[], &[]), vec![]);
    }

    #[test]
    fn test_interleave() {
        assert_eq!(interleave(&[1, 2, 3], &[4, 5, 6]), vec![1, 4, 2, 5, 3, 6]);
        assert_eq!(interleave(&[1], &[2]), vec![1, 2]);
        assert_eq!(interleave(&[], &[]), vec![]);
    }

    #[test]
    fn test_repeat_pattern() {
        assert_eq!(repeat_pattern(&[1, 2], 3), vec![1, 2, 1, 2, 1, 2]);
        assert_eq!(repeat_pattern(&[5], 4), vec![5, 5, 5, 5]);
        assert_eq!(repeat_pattern(&[], 3), vec![]);
        assert_eq!(repeat_pattern(&[1, 2, 3], 0), vec![]);
    }

    #[test]
    fn test_chain_filtered() {
        assert_eq!(
            chain_filtered(&[1, -2, 3], &[-4, 5, 6]),
            vec![1, 3, 5, 6]
        );
        assert_eq!(chain_filtered(&[-1, -2], &[-3, -4]), vec![]);
        assert_eq!(chain_filtered(&[], &[1, 2]), vec![1, 2]);
    }
}
