//! Exercise 18: Option::map_or - Map with default value
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use map_or for transformation with fallback
//! - Provide defaults inline
//! - Simplify common patterns

/// Double value or return 0.
pub fn double_or_zero(value: Option<i32>) -> i32 {
    value.map_or(0, |x| x * 2)
}

/// Get length of optional string or 0.
pub fn length_or_zero(s: Option<&str>) -> usize {
    s.map_or(0, |s| s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_or_zero() {
        assert_eq!(double_or_zero(Some(5)), 10);
        assert_eq!(double_or_zero(Some(-3)), -6);
        assert_eq!(double_or_zero(None), 0);
    }

    #[test]
    fn test_length_or_zero() {
        assert_eq!(length_or_zero(Some("hello")), 5);
        assert_eq!(length_or_zero(Some("")), 0);
        assert_eq!(length_or_zero(None), 0);
    }
}
