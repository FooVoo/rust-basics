//! Exercise 20: Smart Pointer Conversions - Converting between types
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Convert between smart pointer types
//! - Understand From/Into for smart pointers
//! - Work with downcasting and upcasting

use std::rc::Rc;
use std::sync::Arc;

/// Convert Box to Rc.
pub fn box_to_rc<T>(boxed: Box<T>) -> Rc<T> {
    todo!("Implement box_to_rc")
}

/// Convert Box to Arc.
pub fn box_to_arc<T>(boxed: Box<T>) -> Arc<T> {
    todo!("Implement box_to_arc")
}

/// Convert Vec to Box slice.
pub fn vec_to_boxed_slice<T>(vec: Vec<T>) -> Box<[T]> {
    todo!("Implement vec_to_boxed_slice")
}

/// Convert String to Box str.
pub fn string_to_boxed_str(s: String) -> Box<str> {
    todo!("Implement string_to_boxed_str")
}

/// Try to unwrap Rc if it's the only reference.
pub fn try_unwrap_rc<T>(rc: Rc<T>) -> Result<T, Rc<T>> {
    todo!("Implement try_unwrap_rc")
}

/// Try to unwrap Arc if it's the only reference.
pub fn try_unwrap_arc<T>(arc: Arc<T>) -> Result<T, Arc<T>> {
    todo!("Implement try_unwrap_arc")
}

/// Make Rc mutable if possible.
pub fn make_rc_mut<T: Clone>(rc: &mut Rc<T>) -> &mut T {
    todo!("Implement make_rc_mut")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_to_rc() {
        let boxed = Box::new(42);
        let rc = box_to_rc(boxed);
        assert_eq!(*rc, 42);
        assert_eq!(Rc::strong_count(&rc), 1);
    }

    #[test]
    fn test_box_to_arc() {
        let boxed = Box::new(String::from("test"));
        let arc = box_to_arc(boxed);
        assert_eq!(*arc, "test");
        assert_eq!(Arc::strong_count(&arc), 1);
    }

    #[test]
    fn test_vec_to_boxed_slice() {
        let vec = vec![1, 2, 3, 4, 5];
        let slice = vec_to_boxed_slice(vec);
        assert_eq!(&*slice, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_string_to_boxed_str() {
        let s = String::from("hello");
        let boxed = string_to_boxed_str(s);
        assert_eq!(&*boxed, "hello");
    }

    #[test]
    fn test_try_unwrap_rc_success() {
        let rc = Rc::new(42);
        let result = try_unwrap_rc(rc);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_try_unwrap_rc_failure() {
        let rc1 = Rc::new(42);
        let rc2 = Rc::clone(&rc1);
        let result = try_unwrap_rc(rc1);
        assert!(result.is_err());
        assert_eq!(*rc2, 42);
    }

    #[test]
    fn test_try_unwrap_arc_success() {
        let arc = Arc::new(String::from("test"));
        let result = try_unwrap_arc(arc);
        assert_eq!(result, Ok(String::from("test")));
    }

    #[test]
    fn test_try_unwrap_arc_failure() {
        let arc1 = Arc::new(100);
        let arc2 = Arc::clone(&arc1);
        let result = try_unwrap_arc(arc1);
        assert!(result.is_err());
        assert_eq!(*arc2, 100);
    }

    #[test]
    fn test_make_rc_mut() {
        let mut rc = Rc::new(vec![1, 2, 3]);
        let vec_mut = make_rc_mut(&mut rc);
        vec_mut.push(4);
        assert_eq!(*rc, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_make_rc_mut_clones_on_shared() {
        let mut rc1 = Rc::new(vec![1, 2, 3]);
        let rc2 = Rc::clone(&rc1);
        
        // make_mut will clone because rc1 is shared
        let vec_mut = make_rc_mut(&mut rc1);
        vec_mut.push(4);
        
        // rc1 has new data, rc2 has old data
        assert_eq!(*rc1, vec![1, 2, 3, 4]);
        assert_eq!(*rc2, vec![1, 2, 3]);
    }

    #[test]
    fn test_box_from_slice() {
        let arr = [1, 2, 3, 4, 5];
        let boxed: Box<[i32]> = Box::from(&arr[..]);
        assert_eq!(&*boxed, &[1, 2, 3, 4, 5]);
    }
}
