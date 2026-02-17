//! Exercise 10: Struct Lifetimes - Lifetimes in struct definitions
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use lifetime parameters in structs
//! - Store references in structs safely
//! - Understand lifetime constraints on struct instances

#[derive(Debug, PartialEq)]
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!("Implement new")
    }
    
    pub fn get_text(&self) -> &str {
        todo!("Implement get_text")
    }
    
    pub fn len(&self) -> usize {
        todo!("Implement len")
    }
}

/// Create an excerpt from a longer text.
pub fn create_excerpt<'a>(text: &'a str, start: usize, end: usize) -> Excerpt<'a> {
    todo!("Implement create_excerpt")
}

/// Find the longest excerpt from a list.
pub fn longest_excerpt<'a>(excerpts: &'a [Excerpt<'a>]) -> Option<&'a Excerpt<'a>> {
    todo!("Implement longest_excerpt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excerpt_new() {
        let text = "Hello, world!";
        let excerpt = Excerpt::new(text);
        assert_eq!(excerpt.text, "Hello, world!");
    }

    #[test]
    fn test_excerpt_methods() {
        let excerpt = Excerpt::new("test");
        assert_eq!(excerpt.get_text(), "test");
        assert_eq!(excerpt.len(), 4);
    }

    #[test]
    fn test_create_excerpt() {
        let text = "Hello, Rust!";
        let excerpt = create_excerpt(text, 0, 5);
        assert_eq!(excerpt.text, "Hello");
    }

    #[test]
    fn test_longest_excerpt() {
        let e1 = Excerpt::new("short");
        let e2 = Excerpt::new("a much longer text");
        let e3 = Excerpt::new("and");
        
        let excerpts = vec![e1, e2, e3];
        let longest = longest_excerpt(&excerpts);
        assert_eq!(longest.unwrap().text, "a much longer text");
    }

    #[test]
    fn test_excerpt_lifetime() {
        let text = String::from("Hello, world!");
        let excerpt = Excerpt::new(&text);
        assert_eq!(excerpt.text, "Hello, world!");
        // excerpt cannot outlive text
    }
}
