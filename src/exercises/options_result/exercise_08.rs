//! Exercise 08: Option or and or_else
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use or() to provide alternative Options
//! - Use or_else() for lazy alternative computation
//! - Chain fallback options

/// Return the first option if it has a value, otherwise the second.
pub fn first_or_second(first: Option<i32>, second: Option<i32>) -> Option<i32> {
    todo!("Implement first_or_second")
}

/// Try parsing as i32, fallback to parsing as f64 and converting.
pub fn parse_flexible(s: &str) -> Option<i32> {
    todo!("Implement parse_flexible")
}

/// Get value from primary source, fallback to secondary, then to default.
pub fn get_with_fallbacks(
    primary: Option<i32>,
    secondary: Option<i32>,
    default: i32,
) -> i32  {
    todo!("Get value from primary source, fallback to secondary, then to default.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_or_second() {
        assert_eq!(first_or_second(Some(1), Some(2)), Some(1));
        assert_eq!(first_or_second(None, Some(2)), Some(2));
        assert_eq!(first_or_second(Some(1), None), Some(1));
        assert_eq!(first_or_second(None, None), None);
    }

    #[test]
    fn test_parse_flexible() {
        assert_eq!(parse_flexible("42"), Some(42));
        assert_eq!(parse_flexible("42.7"), Some(42));
        assert_eq!(parse_flexible("abc"), None);
    }

    #[test]
    fn test_get_with_fallbacks() {
        assert_eq!(get_with_fallbacks(Some(1), Some(2), 3), 1);
        assert_eq!(get_with_fallbacks(None, Some(2), 3), 2);
        assert_eq!(get_with_fallbacks(None, None, 3), 3);
    }
}
