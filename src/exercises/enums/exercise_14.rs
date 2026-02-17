//! Exercise 14: Enums with Vectors - Collections in Variants
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Store collections inside enum variants
//! - Pattern match to access collection data
//! - Work with complex variant data

#[derive(Debug, PartialEq, Clone)]
pub enum DataSet {
    Empty,
    Single(i32),
    Multiple(Vec<i32>),
}

impl DataSet {
    /// Returns the count of elements in the dataset
    pub fn len(&self) -> usize {
        match self {
            DataSet::Empty => 0,
            DataSet::Single(_) => 1,
            DataSet::Multiple(vec) => vec.len(),
        }
    }

    /// Returns the sum of all elements
    pub fn sum(&self) -> i32 {
        match self {
            DataSet::Empty => 0,
            DataSet::Single(n) => *n,
            DataSet::Multiple(vec) => vec.iter().sum(),
        }
    }

    /// Returns the maximum value if present
    pub fn max(&self) -> Option<i32> {
        match self {
            DataSet::Empty => None,
            DataSet::Single(n) => Some(*n),
            DataSet::Multiple(vec) => vec.iter().max().copied(),
        }
    }

    /// Returns true if the dataset is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, DataSet::Empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        assert_eq!(DataSet::Empty.len(), 0);
        assert_eq!(DataSet::Single(5).len(), 1);
        assert_eq!(DataSet::Multiple(vec![1, 2, 3]).len(), 3);
    }

    #[test]
    fn test_sum() {
        assert_eq!(DataSet::Empty.sum(), 0);
        assert_eq!(DataSet::Single(5).sum(), 5);
        assert_eq!(DataSet::Multiple(vec![1, 2, 3, 4]).sum(), 10);
    }

    #[test]
    fn test_max() {
        assert_eq!(DataSet::Empty.max(), None);
        assert_eq!(DataSet::Single(5).max(), Some(5));
        assert_eq!(DataSet::Multiple(vec![1, 5, 3, 2]).max(), Some(5));
    }

    #[test]
    fn test_is_empty() {
        assert!(DataSet::Empty.is_empty());
        assert!(!DataSet::Single(5).is_empty());
        assert!(!DataSet::Multiple(vec![1, 2]).is_empty());
    }
}
