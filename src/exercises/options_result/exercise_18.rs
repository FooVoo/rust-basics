//! Exercise 18: Chaining Option and Result
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Chain Option and Result operations
//! - Convert between types in chains
//! - Build complex transformation pipelines

/// Find a number, parse it, and validate it's positive.
pub fn find_parse_validate(
    strings: &[&str],
    index: usize,
) -> Result<i32, String> {
    strings
        .get(index)
        .ok_or(String::from("Index out of bounds"))?
        .parse::<i32>()
        .map_err(|e| e.to_string())?
        .checked_sub(0)
        .filter(|&n| n > 0)
        .ok_or(String::from("Number is not positive"))
}

/// Get a character from a string and convert to uppercase.
pub fn get_char_uppercase(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index).map(|c| c.to_ascii_uppercase())
}

/// Parse first element of slice or use default.
pub fn parse_first_or_default(strings: &[&str], default: i32) -> i32 {
    strings
        .first()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_parse_validate() {
        assert_eq!(find_parse_validate(&["5", "10", "15"], 1), Ok(10));
        assert!(find_parse_validate(&["5"], 5).is_err()); // out of bounds
        assert!(find_parse_validate(&["abc"], 0).is_err()); // parse error
        assert!(find_parse_validate(&["0"], 0).is_err()); // not positive
        assert!(find_parse_validate(&["-5"], 0).is_err()); // not positive
    }

    #[test]
    fn test_get_char_uppercase() {
        assert_eq!(get_char_uppercase("hello", 1), Some('E'));
        assert_eq!(get_char_uppercase("rust", 0), Some('R'));
        assert_eq!(get_char_uppercase("hi", 5), None);
    }

    #[test]
    fn test_parse_first_or_default() {
        assert_eq!(parse_first_or_default(&["42", "10"], 0), 42);
        assert_eq!(parse_first_or_default(&[], 99), 99);
        assert_eq!(parse_first_or_default(&["abc"], 99), 99);
    }
}
