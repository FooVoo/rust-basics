//! Exercise 13: Iterator Lifetimes - Working with iterator references
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand lifetimes with iterators
//! - Work with iterator adapters and references
//! - Manage borrowed data through iterator chains

/// Find all strings that start with a given prefix.
pub fn find_with_prefix<'a>(strings: &'a [String], prefix: &str) -> Vec<&'a String> {
    strings.iter().filter(|s| s.starts_with(prefix)).collect()
}

/// Get references to strings longer than a specified length.
pub fn longer_than<'a>(strings: &'a [String], min_len: usize) -> Vec<&'a String> {
    strings.iter().filter(|s| s.len() > min_len).collect()
}

/// Map strings to their first characters.
pub fn first_chars(strings: &[String]) -> Vec<Option<char>> {
    strings.iter().map(|s| s.chars().next()).collect()
}

/// Count strings matching a predicate.
pub fn count_matching<F>(strings: &[String], predicate: F) -> usize
where
    F: Fn(&String) -> bool,
{
    strings.iter().filter(|&s| predicate(s)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_with_prefix() {
        let strings = vec![
            String::from("apple"),
            String::from("apricot"),
            String::from("banana"),
            String::from("avocado"),
        ];
        
        let with_a = find_with_prefix(&strings, "a");
        assert_eq!(with_a.len(), 3);
        assert_eq!(with_a[0], "apple");
    }

    #[test]
    fn test_longer_than() {
        let strings = vec![
            String::from("hi"),
            String::from("hello"),
            String::from("hey"),
            String::from("greetings"),
        ];
        
        let long = longer_than(&strings, 4);
        assert_eq!(long.len(), 2);
    }

    #[test]
    fn test_first_chars() {
        let strings = vec![
            String::from("apple"),
            String::from("banana"),
            String::from(""),
        ];
        
        let chars = first_chars(&strings);
        assert_eq!(chars, vec![Some('a'), Some('b'), None]);
    }

    #[test]
    fn test_count_matching() {
        let strings = vec![
            String::from("test"),
            String::from("testing"),
            String::from("rust"),
            String::from("test123"),
        ];
        
        let count = count_matching(&strings, |s| s.starts_with("test"));
        assert_eq!(count, 3);
    }

    #[test]
    fn test_combined_operations() {
        let strings = vec![
            String::from("apple"),
            String::from("application"),
            String::from("apricot"),
            String::from("banana"),
        ];
        
        let result = find_with_prefix(&strings, "app");
        // Count how many are longer than 5
        let count = result.iter().filter(|s| s.len() > 5).count();
        assert_eq!(count, 1);
    }
}
