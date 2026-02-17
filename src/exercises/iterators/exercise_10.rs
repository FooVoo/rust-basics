//! Exercise 10: Enumerate and Zip - Working with indices
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use enumerate() to get indices
//! - Use zip() to combine iterators
//! - Work with tuples in iterations

/// Get indices of elements that match a predicate.
pub fn indices_of_evens(numbers: &[i32]) -> Vec<usize> {
    todo!("Implement indices_of_evens")
}

/// Create pairs of elements from two slices.
pub fn zip_slices(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    todo!("Implement zip_slices")
}

/// Sum corresponding elements from two slices.
pub fn sum_corresponding(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!("Implement sum_corresponding")
}

/// Find indices where two slices differ.
pub fn diff_indices(a: &[i32], b: &[i32]) -> Vec<usize> {
    todo!("Implement diff_indices")
}

/// Create indexed strings.
pub fn indexed_strings(strings: &[&str]) -> Vec<String> {
    todo!("Implement indexed_strings")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indices_of_evens() {
        assert_eq!(indices_of_evens(&[1, 2, 3, 4, 5, 6]), vec![1, 3, 5]);
        assert_eq!(indices_of_evens(&[1, 3, 5]), vec![]);
        assert_eq!(indices_of_evens(&[]), vec![]);
    }

    #[test]
    fn test_zip_slices() {
        assert_eq!(zip_slices(&[1, 2, 3], &[4, 5, 6]), vec![(1, 4), (2, 5), (3, 6)]);
        assert_eq!(zip_slices(&[1, 2], &[3]), vec![(1, 3)]);
        assert_eq!(zip_slices(&[], &[1, 2]), vec![]);
    }

    #[test]
    fn test_sum_corresponding() {
        assert_eq!(sum_corresponding(&[1, 2, 3], &[4, 5, 6]), vec![5, 7, 9]);
        assert_eq!(sum_corresponding(&[1], &[2, 3]), vec![3]);
        assert_eq!(sum_corresponding(&[], &[]), vec![]);
    }

    #[test]
    fn test_diff_indices() {
        assert_eq!(diff_indices(&[1, 2, 3, 4], &[1, 5, 3, 6]), vec![1, 3]);
        assert_eq!(diff_indices(&[1, 2, 3], &[1, 2, 3]), vec![]);
        assert_eq!(diff_indices(&[], &[]), vec![]);
    }

    #[test]
    fn test_indexed_strings() {
        assert_eq!(
            indexed_strings(&["a", "b", "c"]),
            vec!["0: a", "1: b", "2: c"]
        );
        assert_eq!(indexed_strings(&[]), Vec::<String>::new());
    }
}
