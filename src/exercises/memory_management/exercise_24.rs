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
    pub fn new(content: &'a str) -> Self {
        Parser {
            content,
            position: 0,
        }
    }
    
    pub fn take_while<F>(&mut self, predicate: F) -> &'a str
    where
        F: Fn(char) -> bool,
    {
        let start = self.position;
        while self.position < self.content.len() {
            if let Some(ch) = self.content[self.position..].chars().next() {
                if predicate(ch) {
                    self.position += ch.len_utf8();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        &self.content[start..self.position]
    }
    
    pub fn skip_whitespace(&mut self) {
        self.take_while(|c| c.is_whitespace());
    }
    
    pub fn remaining(&self) -> &'a str {
        &self.content[self.position..]
    }
}

/// Parse words from text.
pub fn parse_words<'a>(parser: &mut Parser<'a>) -> Vec<&'a str> {
    let mut words = Vec::new();
    loop {
        parser.skip_whitespace();
        if parser.remaining().is_empty() {
            break;
        }
        let word = parser.take_while(|c| !c.is_whitespace());
        if !word.is_empty() {
            words.push(word);
        }
    }
    words
}

pub struct Splitter<'text> {
    text: &'text str,
    delimiter: char,
}

impl<'text> Splitter<'text> {
    pub fn new(text: &'text str, delimiter: char) -> Self {
        Splitter { text, delimiter }
    }
    
    pub fn split(&self) -> Vec<&'text str> {
        self.text.split(self.delimiter).collect()
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
