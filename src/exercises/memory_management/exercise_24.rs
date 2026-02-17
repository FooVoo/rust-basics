//! Exercise 24: Lifetime Subtyping - Understanding lifetime variance
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand lifetime subtyping
//! - Work with lifetime variance
//! - Use lifetime bounds effectively

pub struct Parser<'a> {
    content: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self  {
        todo!("Implement new")
    }
    
    pub fn take_while<F>(&mut self, predicate: F) -> &'a str
    where
        F: Fn(char) -> bool,
     {
        todo!("Implement take_while")
    }
    
    pub fn skip_whitespace(&mut self)  {
        todo!("Implement skip_whitespace")
    }
    
    pub fn remaining(&self) -> &'a str  {
        todo!("Implement remaining")
    }
}

/// Parse words from text.
pub fn parse_words<'a>(parser: &mut Parser<'a>) -> Vec<&'a str>  {
    todo!("Parse words from text.")
}

pub struct Splitter<'text> {
    text: &'text str,
    delimiter: char,
}

impl<'text> Splitter<'text> {
    pub fn new(text: &'text str, delimiter: char) -> Self  {
        todo!("Implement new")
    }
    
    pub fn split(&self) -> Vec<&'text str>  {
        todo!("Implement split")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let content = "test";
        let parser = Parser::new(content);
        assert_eq!(parser.remaining(), "test");
    }

    #[test]
    fn test_take_while() {
        let content = "abc123def";
        let mut parser = Parser::new(content);
        let letters = parser.take_while(|c| c.is_alphabetic());
        assert_eq!(letters, "abc");
        assert_eq!(parser.remaining(), "123def");
    }

    #[test]
    fn test_skip_whitespace() {
        let content = "   hello";
        let mut parser = Parser::new(content);
        parser.skip_whitespace();
        assert_eq!(parser.remaining(), "hello");
    }

    #[test]
    fn test_parse_words() {
        let content = "hello world rust";
        let mut parser = Parser::new(content);
        let words = parse_words(&mut parser);
        assert_eq!(words, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_parse_words_with_spacing() {
        let content = "  one   two  three  ";
        let mut parser = Parser::new(content);
        let words = parse_words(&mut parser);
        assert_eq!(words, vec!["one", "two", "three"]);
    }

    #[test]
    fn test_splitter() {
        let text = "a,b,c,d";
        let splitter = Splitter::new(text, ',');
        let parts = splitter.split();
        assert_eq!(parts, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_lifetime_bounds() {
        let text = String::from("test input data");
        let mut parser = Parser::new(&text);
        let word1 = parser.take_while(|c| !c.is_whitespace());
        parser.skip_whitespace();
        let word2 = parser.take_while(|c| !c.is_whitespace());
        
        assert_eq!(word1, "test");
        assert_eq!(word2, "input");
    }
}
