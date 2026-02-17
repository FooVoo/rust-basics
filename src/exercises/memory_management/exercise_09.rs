//! Exercise 09: Basic Lifetimes - Understanding lifetime annotations
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand lifetime parameters
//! - Use lifetime annotations in function signatures
//! - Learn when lifetimes are needed

/// Return the longer of two string slices.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str  {
    todo!("Return the longer of two string slices.")
}

/// Get the first element of a slice if it exists.
pub fn first_element<'a, T>(slice: &'a [T]) -> Option<&'a T>  {
    todo!("Get the first element of a slice if it exists.")
}

/// Concatenate two strings with a separator.
pub fn concat_with_sep<'a>(a: &'a str, b: &'a str, sep: &str) -> String  {
    todo!("Concatenate two strings with a separator.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let s1 = "short";
        let s2 = "longer string";
        assert_eq!(longest(s1, s2), "longer string");
        assert_eq!(longest(s2, s1), "longer string");
    }

    #[test]
    fn test_longest_equal() {
        let s1 = "same";
        let s2 = "size";
        let result = longest(s1, s2);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_first_element() {
        let v = vec![1, 2, 3];
        assert_eq!(first_element(&v), Some(&1));
        
        let empty: Vec<i32> = vec![];
        assert_eq!(first_element(&empty), None);
    }

    #[test]
    fn test_concat_with_sep() {
        assert_eq!(concat_with_sep("hello", "world", " "), "hello world");
        assert_eq!(concat_with_sep("foo", "bar", "-"), "foo-bar");
    }

    #[test]
    fn test_lifetime_scope() {
        let string1 = String::from("long string");
        let string2 = String::from("short");
        let result = longest(&string1, &string2);
        // result is valid as long as both strings are in scope
        assert!(result.len() > 0);
    }
}
