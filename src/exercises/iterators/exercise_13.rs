//! Exercise 13: Windows and Chunks - Grouped iteration
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use windows() for sliding windows
//! - Use chunks() for fixed-size groups
//! - Process elements in groups

/// Sum of each pair of consecutive numbers.
pub fn pairwise_sum(numbers: &[i32]) -> Vec<i32> {
    numbers.windows(2).map(|w| w[0] + w[1]).collect()
}

/// Find local maxima (element greater than neighbors).
pub fn local_maxima(numbers: &[i32]) -> Vec<i32> {
    if numbers.len() < 3 {
        return vec![];
    }
    
    numbers
        .windows(3)
        .filter_map(|w| {
            if w[1] > w[0] && w[1] > w[2] {
                Some(w[1])
            } else {
                None
            }
        })
        .collect()
}

/// Group numbers into chunks and sum each chunk.
pub fn chunk_sums(numbers: &[i32], chunk_size: usize) -> Vec<i32> {
    numbers
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().sum())
        .collect()
}

/// Check if any consecutive triple sums to target.
pub fn has_triple_sum(numbers: &[i32], target: i32) -> bool {
    numbers
        .windows(3)
        .any(|w| w[0] + w[1] + w[2] == target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairwise_sum() {
        assert_eq!(pairwise_sum(&[1, 2, 3, 4]), vec![3, 5, 7]);
        assert_eq!(pairwise_sum(&[1]), vec![]);
        assert_eq!(pairwise_sum(&[]), vec![]);
        assert_eq!(pairwise_sum(&[5, 10]), vec![15]);
    }

    #[test]
    fn test_local_maxima() {
        assert_eq!(local_maxima(&[1, 3, 2, 5, 4]), vec![3, 5]);
        assert_eq!(local_maxima(&[1, 2, 3, 4]), vec![]);
        assert_eq!(local_maxima(&[1, 2]), vec![]);
        assert_eq!(local_maxima(&[1, 5, 2, 3, 1, 4, 2]), vec![5, 3, 4]);
    }

    #[test]
    fn test_chunk_sums() {
        assert_eq!(chunk_sums(&[1, 2, 3, 4, 5, 6], 2), vec![3, 7, 11]);
        assert_eq!(chunk_sums(&[1, 2, 3, 4, 5], 2), vec![3, 7, 5]);
        assert_eq!(chunk_sums(&[], 2), vec![]);
        assert_eq!(chunk_sums(&[1, 2, 3], 3), vec![6]);
    }

    #[test]
    fn test_has_triple_sum() {
        assert!(has_triple_sum(&[1, 2, 3, 4], 6));
        assert!(!has_triple_sum(&[1, 2, 3, 4], 20));
        assert!(!has_triple_sum(&[1, 2], 3));
        assert!(has_triple_sum(&[5, 5, 5], 15));
    }
}
