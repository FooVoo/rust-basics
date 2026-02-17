//! Exercise 09: Rc Basics - Understand reference counting
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand Rc<T> for shared ownership
//! - Learn reference counting mechanics
//! - Work with Rc::clone and strong_count

use std::rc::Rc;

/// Create a shared reference-counted value.
pub fn create_shared(value: i32) -> Rc<i32> {
    todo!("Implement create_shared")
}

/// Clone an Rc to create another reference.
pub fn clone_rc(rc: &Rc<i32>) -> Rc<i32> {
    todo!("Implement clone_rc")
}

/// Get the strong count of an Rc.
pub fn get_count(rc: &Rc<i32>) -> usize {
    todo!("Implement get_count")
}

/// Create multiple references to the same data.
pub fn create_multiple_refs(value: i32, count: usize) -> Vec<Rc<i32>> {
    todo!("Implement create_multiple_refs")
}

/// Share data between two vectors.
pub fn share_between_vecs() -> (Vec<Rc<String>>, Vec<Rc<String>>) {
    todo!("Implement share_between_vecs")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shared() {
        let rc = create_shared(42);
        assert_eq!(*rc, 42);
        assert_eq!(Rc::strong_count(&rc), 1);
    }

    #[test]
    fn test_clone_rc() {
        let rc1 = create_shared(42);
        let rc2 = clone_rc(&rc1);
        assert_eq!(*rc1, *rc2);
        assert_eq!(Rc::strong_count(&rc1), 2);
        assert_eq!(Rc::strong_count(&rc2), 2);
    }

    #[test]
    fn test_get_count() {
        let rc = create_shared(42);
        assert_eq!(get_count(&rc), 1);
        let _rc2 = clone_rc(&rc);
        assert_eq!(get_count(&rc), 2);
    }

    #[test]
    fn test_create_multiple_refs() {
        let refs = create_multiple_refs(100, 5);
        assert_eq!(refs.len(), 5);
        // All references point to same value
        for r in &refs {
            assert_eq!(**r, 100);
        }
        // Count should be 5
        assert_eq!(Rc::strong_count(&refs[0]), 5);
    }

    #[test]
    fn test_share_between_vecs() {
        let (vec1, vec2) = share_between_vecs();
        assert_eq!(vec1.len(), 2);
        assert_eq!(vec2.len(), 1);
        // All references point to same string
        assert_eq!(&**vec1[0], "shared");
        assert_eq!(&**vec2[0], "shared");
        // Total count should be 3
        assert_eq!(Rc::strong_count(&vec1[0]), 3);
    }

    #[test]
    fn test_rc_drop() {
        let rc1 = create_shared(42);
        {
            let rc2 = clone_rc(&rc1);
            assert_eq!(Rc::strong_count(&rc1), 2);
            drop(rc2);
        }
        assert_eq!(Rc::strong_count(&rc1), 1);
    }
}
