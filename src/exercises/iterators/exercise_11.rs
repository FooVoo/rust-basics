//! Exercise 11: Scan - Stateful transformation
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use scan() for stateful iterations
//! - Understand the difference between fold and scan
//! - Maintain state across iterations

/// Running sum of numbers.
pub fn running_sum(numbers: &[i32]) -> Vec<i32> {
    todo!("Implement running_sum")
}

/// Running product of numbers.
pub fn running_product(numbers: &[i32]) -> Vec<i32> {
    todo!("Implement running_product")
}

/// Running maximum.
pub fn running_max(numbers: &[i32]) -> Vec<i32> {
    todo!("Implement running_max")
}

/// Count occurrences up to each position.
pub fn running_count(items: &[char], target: char) -> Vec<usize> {
    todo!("Implement running_count")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_sum(&[]), vec![]);
        assert_eq!(running_sum(&[5]), vec![5]);
        assert_eq!(running_sum(&[1, -1, 1, -1]), vec![1, 0, 1, 0]);
    }

    #[test]
    fn test_running_product() {
        assert_eq!(running_product(&[1, 2, 3, 4]), vec![1, 2, 6, 24]);
        assert_eq!(running_product(&[]), vec![]);
        assert_eq!(running_product(&[5]), vec![5]);
        assert_eq!(running_product(&[2, 0, 3]), vec![2, 0, 0]);
    }

    #[test]
    fn test_running_max() {
        assert_eq!(running_max(&[1, 5, 3, 7, 2]), vec![1, 5, 5, 7, 7]);
        assert_eq!(running_max(&[5, 4, 3, 2, 1]), vec![5, 5, 5, 5, 5]);
        assert_eq!(running_max(&[1]), vec![1]);
    }

    #[test]
    fn test_running_count() {
        assert_eq!(running_count(&['a', 'b', 'a', 'c', 'a'], 'a'), vec![1, 1, 2, 2, 3]);
        assert_eq!(running_count(&[], 'x'), vec![]);
        assert_eq!(running_count(&['a', 'a', 'a'], 'a'), vec![1, 2, 3]);
    }
}
