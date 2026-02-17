//! Exercise 13: Result::ok - Convert Result to Option
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Convert Result to Option
//! - Discard error information
//! - Simplify when errors don't matter

/// Parse multiple strings, keep only successful ones.
pub fn parse_all_valid(strings: &[&str]) -> Vec<i32> {
    todo!("Implement parse_all_valid")
}

/// Try operations and collect successes.
pub fn safe_divisions(numerators: &[i32], denominator: i32) -> Vec<i32> {
    todo!("Implement safe_divisions")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_valid() {
        assert_eq!(parse_all_valid(&["1", "2", "abc", "3"]), vec![1, 2, 3]);
        assert_eq!(parse_all_valid(&["10", "20", "30"]), vec![10, 20, 30]);
        assert_eq!(parse_all_valid(&["abc", "xyz"]), vec![]);
    }

    #[test]
    fn test_safe_divisions() {
        assert_eq!(safe_divisions(&[10, 20, 30], 5), vec![2, 4, 6]);
        assert_eq!(safe_divisions(&[10, 20, 30], 0), vec![]);
        assert_eq!(safe_divisions(&[], 5), vec![]);
    }
}
