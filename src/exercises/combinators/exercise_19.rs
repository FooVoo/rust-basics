//! Exercise 19: Option::map_or_else - Map with lazy default
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use map_or_else for lazy default computation
//! - Optimize with lazy evaluation
//! - Handle expensive fallbacks

/// Get value squared or compute from slice.
pub fn square_or_sum(value: Option<i32>, fallback: &[i32]) -> i32 {
    todo!("Implement square_or_sum")
}

/// Parse or get default from function.
pub fn parse_or_generate(s: &str, generator: fn() -> i32) -> i32 {
    todo!("Implement parse_or_generate")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_or_sum() {
        assert_eq!(square_or_sum(Some(5), &[1, 2, 3]), 25);
        assert_eq!(square_or_sum(None, &[1, 2, 3]), 6);
        assert_eq!(square_or_sum(None, &[]), 0);
    }

    #[test]
    fn test_parse_or_generate() {
        assert_eq!(parse_or_generate("42", || 10), 42);
        assert_eq!(parse_or_generate("abc", || 10), 10);
        assert_eq!(parse_or_generate("", || 5 + 5), 10);
    }
}
