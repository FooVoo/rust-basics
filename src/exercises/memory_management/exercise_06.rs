//! Exercise 06: Reference Patterns - Pattern matching with references
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use references in pattern matching
//! - Understand ref and ref mut patterns
//! - Work with borrowed values in match expressions

/// Match on a reference and return a description.
pub fn describe_number(n: &i32) -> String {
    match n {
        0 => String::from("zero"),
        1..=10 => String::from("small"),
        11..=100 => String::from("medium"),
        _ => String::from("large"),
    }
}

/// Find the first even number in a slice.
pub fn first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&n| n % 2 == 0).copied()
}

/// Classify a string by length.
pub fn classify_by_length(s: &str) -> &'static str {
    match s.len() {
        0 => "empty",
        1..=5 => "short",
        6..=15 => "medium",
        _ => "long",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_number() {
        assert_eq!(describe_number(&0), "zero");
        assert_eq!(describe_number(&5), "small");
        assert_eq!(describe_number(&50), "medium");
        assert_eq!(describe_number(&200), "large");
    }

    #[test]
    fn test_first_even() {
        assert_eq!(first_even(&[1, 3, 4, 5]), Some(4));
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_even(&[2, 4, 6]), Some(2));
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn test_classify_by_length() {
        assert_eq!(classify_by_length(""), "empty");
        assert_eq!(classify_by_length("hi"), "short");
        assert_eq!(classify_by_length("hello"), "short");
        assert_eq!(classify_by_length("hello world"), "medium");
        assert_eq!(classify_by_length("this is a very long string"), "long");
    }

    #[test]
    fn test_reference_reuse() {
        let num = 42;
        let desc1 = describe_number(&num);
        let desc2 = describe_number(&num);
        assert_eq!(desc1, desc2);
        assert_eq!(num, 42); // Original still valid
    }
}
