//! Exercise 07: Option and_then (flatMap)
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use and_then() to chain Option-returning operations
//! - Understand the difference between map and and_then
//! - Avoid nested Options

/// Parse a string to i32, then check if it's positive.
pub fn parse_positive(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().and_then(|n| {
        if n > 0 {
            Some(n)
        } else {
            None
        }
    })
}

/// Get the first element, then check if it's even.
pub fn first_even(numbers: &[i32]) -> Option<i32> {
    numbers.first().copied().and_then(|n| {
        if n % 2 == 0 {
            Some(n)
        } else {
            None
        }
    })
}

/// Safely divide two optional numbers.
pub fn safe_divide(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.and_then(|x| b.and_then(|y| if y != 0 { Some(x / y) } else { None }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Some(42));
        assert_eq!(parse_positive("0"), None);
        assert_eq!(parse_positive("-5"), None);
        assert_eq!(parse_positive("abc"), None);
    }

    #[test]
    fn test_first_even() {
        assert_eq!(first_even(&[2, 3, 4]), Some(2));
        assert_eq!(first_even(&[1, 2, 3]), None);
        assert_eq!(first_even(&[]), None);
        assert_eq!(first_even(&[0]), Some(0));
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(Some(10), Some(2)), Some(5));
        assert_eq!(safe_divide(Some(10), Some(0)), None);
        assert_eq!(safe_divide(None, Some(2)), None);
        assert_eq!(safe_divide(Some(10), None), None);
        assert_eq!(safe_divide(None, None), None);
    }
}
