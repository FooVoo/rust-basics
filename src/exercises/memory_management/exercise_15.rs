//! Exercise 15: Lifetime Elision - Understanding implicit lifetimes
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand when lifetime annotations can be elided
//! - Learn lifetime elision rules
//! - Recognize patterns that don't need explicit lifetimes

/// Return a string slice from input (lifetime elision applies).
pub fn trim_spaces(s: &str) -> &str  {
    todo!("Return a string slice from input (lifetime elision applies).")
}

/// Get first n characters of a string.
pub fn take_prefix(s: &str, n: usize) -> &str  {
    todo!("Get first n characters of a string.")
}

/// Extract substring between delimiters.
pub fn extract_between<'a>(s: &'a str, start: &str, end: &str) -> Option<&'a str>  {
    todo!("Extract substring between delimiters.")
}

pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self  {
        todo!("Implement new")
    }
    
    pub fn peek(&self) -> Option<char>  {
        todo!("Implement peek")
    }
    
    pub fn remaining(&self) -> &str  {
        todo!("Implement remaining")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_spaces() {
        assert_eq!(trim_spaces("  hello  "), "hello");
        assert_eq!(trim_spaces("test"), "test");
    }

    #[test]
    fn test_take_prefix() {
        assert_eq!(take_prefix("hello", 3), "hel");
        assert_eq!(take_prefix("hi", 10), "hi");
        assert_eq!(take_prefix("test", 4), "test");
    }

    #[test]
    fn test_extract_between() {
        let text = "Hello [world] test";
        assert_eq!(extract_between(text, "[", "]"), Some("world"));
        
        let text2 = "no delimiters";
        assert_eq!(extract_between(text2, "[", "]"), None);
    }

    #[test]
    fn test_parser_new() {
        let parser = Parser::new("test input");
        assert_eq!(parser.remaining(), "test input");
    }

    #[test]
    fn test_parser_peek() {
        let parser = Parser::new("abc");
        assert_eq!(parser.peek(), Some('a'));
    }

    #[test]
    fn test_parser_lifetime() {
        let input = String::from("parse this");
        let parser = Parser::new(&input);
        let remaining = parser.remaining();
        assert_eq!(remaining, "parse this");
    }
}
