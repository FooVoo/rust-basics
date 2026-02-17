//! Exercise 07: Vector Ownership - Managing vectors and their elements
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand ownership with collections
//! - Modify vectors through mutable references
//! - Work with vector elements safely

/// Add an element to a vector.
pub fn add_element(v: &mut Vec<i32>, element: i32) {
    todo!("Implement add_element")
}

/// Remove and return the last element from a vector.
pub fn remove_last(v: &mut Vec<i32>) -> Option<i32> {
    todo!("Implement remove_last")
}

/// Create a new vector with doubled values.
pub fn double_values(v: &[i32]) -> Vec<i32> {
    todo!("Implement double_values")
}

/// Sum all elements in a vector.
pub fn sum_vector(v: &[i32]) -> i32 {
    todo!("Implement sum_vector")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_element() {
        let mut v = vec![1, 2, 3];
        add_element(&mut v, 4);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_last() {
        let mut v = vec![1, 2, 3];
        assert_eq!(remove_last(&mut v), Some(3));
        assert_eq!(v, vec![1, 2]);
        
        let mut empty: Vec<i32> = vec![];
        assert_eq!(remove_last(&mut empty), None);
    }

    #[test]
    fn test_double_values() {
        let v = vec![1, 2, 3, 4];
        let doubled = double_values(&v);
        assert_eq!(doubled, vec![2, 4, 6, 8]);
        assert_eq!(v, vec![1, 2, 3, 4]); // Original unchanged
    }

    #[test]
    fn test_sum_vector() {
        assert_eq!(sum_vector(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_vector(&[]), 0);
        assert_eq!(sum_vector(&[5]), 5);
    }

    #[test]
    fn test_vector_operations() {
        let mut v = vec![1, 2];
        add_element(&mut v, 3);
        let sum = sum_vector(&v);
        assert_eq!(sum, 6);
        
        let doubled = double_values(&v);
        assert_eq!(doubled, vec![2, 4, 6]);
    }
}
