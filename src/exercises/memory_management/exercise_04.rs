//! Exercise 04: Clone vs Copy - Understanding deep copying
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use the clone() method to create deep copies
//! - Understand when cloning is necessary
//! - Distinguish between Copy and Clone traits

/// Clone a string and modify the clone.
pub fn clone_and_modify(s: &String, suffix: &str) -> String {
    let mut cloned = s.clone();
    cloned.push_str(suffix);
    cloned
}

/// Create a vector of cloned strings from a slice.
pub fn clone_all(strings: &[String]) -> Vec<String> {
    strings.iter().map(|s| s.clone()).collect()
}

/// Count occurrences of a character in a string (using clone for demonstration).
pub fn count_char_in_clone(s: &String, ch: char) -> usize {
    let cloned = s.clone();
    cloned.chars().filter(|&c| c == ch).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_and_modify() {
        let original = String::from("hello");
        let modified = clone_and_modify(&original, " world");
        
        assert_eq!(original, "hello"); // Original unchanged
        assert_eq!(modified, "hello world"); // Clone modified
    }

    #[test]
    fn test_clone_all() {
        let strings = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ];
        let cloned = clone_all(&strings);
        
        assert_eq!(strings.len(), cloned.len());
        assert_eq!(strings, cloned);
    }

    #[test]
    fn test_count_char_in_clone() {
        let s = String::from("hello world");
        let count = count_char_in_clone(&s, 'l');
        assert_eq!(count, 3);
        assert_eq!(s, "hello world"); // Original unchanged
    }

    #[test]
    fn test_independent_clones() {
        let original = String::from("rust");
        let clone1 = clone_and_modify(&original, "1");
        let clone2 = clone_and_modify(&original, "2");
        
        assert_eq!(original, "rust");
        assert_eq!(clone1, "rust1");
        assert_eq!(clone2, "rust2");
    }
}
