//! Exercise 03: Mutable Borrowing - Modify data through references
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use mutable references to modify borrowed data
//! - Understand that only one mutable borrow is allowed at a time
//! - Learn the difference between &T and &mut T

/// Append a string to the end of another string.
pub fn append_string(s: &mut String, suffix: &str)  {
    todo!("Append a string to the end of another string.")
}

/// Clear a string (make it empty).
pub fn clear_string(s: &mut String)  {
    todo!("Clear a string (make it empty).")
}

/// Convert a string to uppercase in place.
pub fn uppercase_in_place(s: &mut String)  {
    todo!("Convert a string to uppercase in place.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_string() {
        let mut s = String::from("hello");
        append_string(&mut s, " world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_clear_string() {
        let mut s = String::from("test");
        clear_string(&mut s);
        assert_eq!(s, "");
        assert!(s.is_empty());
    }

    #[test]
    fn test_uppercase_in_place() {
        let mut s = String::from("hello");
        uppercase_in_place(&mut s);
        assert_eq!(s, "HELLO");
    }

    #[test]
    fn test_multiple_mutations() {
        let mut s = String::from("rust");
        append_string(&mut s, " is");
        append_string(&mut s, " awesome");
        uppercase_in_place(&mut s);
        assert_eq!(s, "RUST IS AWESOME");
    }

    #[test]
    fn test_mutation_after_borrow() {
        let mut s = String::from("test");
        append_string(&mut s, "123");
        uppercase_in_place(&mut s);
        clear_string(&mut s);
        assert!(s.is_empty());
    }
}
