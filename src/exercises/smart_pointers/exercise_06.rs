//! Exercise 06: Box Deref - Understand automatic dereferencing
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Understand Deref trait and deref coercion
//! - Work with Box dereference operations
//! - Learn automatic dereferencing

/// Demonstrate deref coercion with Box.
pub fn deref_example(boxed: Box<String>) -> usize {
    // Box<String> automatically derefs to &String, then to &str
    boxed.len()
}

/// Use Box with method calls.
pub fn box_method_call(boxed: Box<Vec<i32>>) -> Option<i32> {
    boxed.first().copied()
}

/// Compare boxed values.
pub fn compare_boxes(a: Box<i32>, b: Box<i32>) -> bool {
    *a == *b
}

/// Clone the inner value from a Box.
pub fn clone_from_box(boxed: Box<String>) -> String {
    (*boxed).clone()
}

/// Modify through mutable box.
pub fn modify_boxed(mut boxed: Box<Vec<i32>>, value: i32) -> Box<Vec<i32>> {
    boxed.push(value);
    boxed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_example() {
        let s = Box::new(String::from("hello"));
        assert_eq!(deref_example(s), 5);
    }

    #[test]
    fn test_box_method_call() {
        let v = Box::new(vec![1, 2, 3]);
        assert_eq!(box_method_call(v), Some(1));

        let empty = Box::new(vec![]);
        assert_eq!(box_method_call(empty), None);
    }

    #[test]
    fn test_compare_boxes() {
        assert!(compare_boxes(Box::new(42), Box::new(42)));
        assert!(!compare_boxes(Box::new(42), Box::new(43)));
    }

    #[test]
    fn test_clone_from_box() {
        let boxed = Box::new(String::from("test"));
        let cloned = clone_from_box(boxed);
        assert_eq!(cloned, "test");
    }

    #[test]
    fn test_modify_boxed() {
        let v = Box::new(vec![1, 2]);
        let modified = modify_boxed(v, 3);
        assert_eq!(*modified, vec![1, 2, 3]);
    }
}
