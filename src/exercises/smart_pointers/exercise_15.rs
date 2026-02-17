//! Exercise 15: Arc Basics - Thread-safe reference counting
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand Arc<T> for thread-safe shared ownership
//! - Compare Arc vs Rc
//! - Work with Arc across threads

use std::sync::Arc;

/// Create a shared Arc value.
pub fn create_arc(value: i32) -> Arc<i32> {
    todo!("Implement create_arc")
}

/// Clone an Arc reference.
pub fn clone_arc(arc: &Arc<i32>) -> Arc<i32> {
    todo!("Implement clone_arc")
}

/// Get strong count of an Arc.
pub fn arc_count(arc: &Arc<i32>) -> usize {
    todo!("Implement arc_count")
}

/// Share data across multiple "threads" (simulated with Vec).
pub fn share_across_contexts(value: String, count: usize) -> Vec<Arc<String>> {
    todo!("Implement share_across_contexts")
}

/// Demonstrate Arc with large data.
pub struct LargeData {
    pub data: Vec<i32>,
}

impl LargeData {
    pub fn new(size: usize) -> Arc<Self> {
        todo!("Implement new")
    }

    pub fn sum(&self) -> i64 {
        todo!("Implement sum")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_arc() {
        let arc = create_arc(42);
        assert_eq!(*arc, 42);
        assert_eq!(Arc::strong_count(&arc), 1);
    }

    #[test]
    fn test_clone_arc() {
        let arc1 = create_arc(42);
        let arc2 = clone_arc(&arc1);
        assert_eq!(*arc1, *arc2);
        assert_eq!(Arc::strong_count(&arc1), 2);
    }

    #[test]
    fn test_arc_count() {
        let arc = create_arc(100);
        assert_eq!(arc_count(&arc), 1);
        let _arc2 = clone_arc(&arc);
        assert_eq!(arc_count(&arc), 2);
        let _arc3 = clone_arc(&arc);
        assert_eq!(arc_count(&arc), 3);
    }

    #[test]
    fn test_share_across_contexts() {
        let refs = share_across_contexts("shared".to_string(), 10);
        assert_eq!(refs.len(), 10);
        for r in &refs {
            assert_eq!(**r, "shared");
        }
        assert_eq!(Arc::strong_count(&refs[0]), 10);
    }

    #[test]
    fn test_large_data() {
        let data = LargeData::new(100);
        assert_eq!(data.data.len(), 100);
        let expected_sum = (0..100i64).sum();
        assert_eq!(data.sum(), expected_sum);
    }

    #[test]
    fn test_arc_is_send_sync() {
        // Arc implements Send and Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        
        assert_send::<Arc<i32>>();
        assert_sync::<Arc<i32>>();
    }

    #[test]
    fn test_arc_with_vec() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data2 = Arc::clone(&data);
        let data3 = Arc::clone(&data);
        
        assert_eq!(*data, vec![1, 2, 3, 4, 5]);
        assert_eq!(*data2, vec![1, 2, 3, 4, 5]);
        assert_eq!(*data3, vec![1, 2, 3, 4, 5]);
        assert_eq!(Arc::strong_count(&data), 3);
    }
}
