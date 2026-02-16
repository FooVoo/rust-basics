//! Exercise 15: Option::flatten - Flatten nested Options
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Flatten Option<Option<T>> to Option<T>
//! - Handle nested optional structures
//! - Simplify complex Option chains

/// Parse a string that might contain "None" keyword.
pub fn parse_or_none_keyword(s: &str) -> Option<Option<i32>> {
    if s == "None" {
        Some(None)
    } else {
        Some(s.parse::<i32>().ok())
    }
}

/// Get nested optional value flattened.
pub fn get_flattened(s: &str) -> Option<i32> {
    parse_or_none_keyword(s).flatten()
}

/// Find and parse in one operation.
pub fn find_and_parse(strings: &[&str], target: &str) -> Option<i32> {
    strings
        .iter()
        .find(|&&s| s == target)
        .map(|s| s.parse::<i32>().ok())
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_flattened() {
        assert_eq!(get_flattened("42"), Some(42));
        assert_eq!(get_flattened("None"), None);
        assert_eq!(get_flattened("abc"), None);
    }

    #[test]
    fn test_find_and_parse() {
        assert_eq!(find_and_parse(&["10", "20", "30"], "20"), Some(20));
        assert_eq!(find_and_parse(&["10", "20", "30"], "40"), None);
        assert_eq!(find_and_parse(&["10", "abc", "30"], "abc"), None);
    }
}
