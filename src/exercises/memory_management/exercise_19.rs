//! Exercise 19: Method Lifetimes - Lifetimes in method signatures
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use lifetimes in method definitions
//! - Understand self lifetime relationships
//! - Work with methods that return references

pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    pub fn new(text: &'a str) -> Self  {
        todo!("Implement new")
    }
    
    pub fn get_text(&self) -> &str  {
        todo!("Implement get_text")
    }
    
    pub fn find_word(&self, word: &str) -> Option<usize>  {
        todo!("Implement find_word")
    }
    
    pub fn get_line(&self, n: usize) -> Option<&str>  {
        todo!("Implement get_line")
    }
    
    pub fn first_sentence(&self) -> &str  {
        todo!("Implement first_sentence")
    }
}

/// Count lines in text.
pub fn count_lines(analyzer: &TextAnalyzer) -> usize  {
    todo!("Count lines in text.")
}

/// Get all words from text.
pub fn extract_words(text: &str) -> Vec<&str>  {
    todo!("Get all words from text.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_analyzer_new() {
        let text = "Hello, world!";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.get_text(), "Hello, world!");
    }

    #[test]
    fn test_find_word() {
        let analyzer = TextAnalyzer::new("hello world hello");
        assert_eq!(analyzer.find_word("world"), Some(6));
        assert_eq!(analyzer.find_word("rust"), None);
    }

    #[test]
    fn test_get_line() {
        let text = "line 1\nline 2\nline 3";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.get_line(0), Some("line 1"));
        assert_eq!(analyzer.get_line(1), Some("line 2"));
        assert_eq!(analyzer.get_line(5), None);
    }

    #[test]
    fn test_first_sentence() {
        let text = "First sentence. Second sentence.";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(analyzer.first_sentence(), "First sentence.");
    }

    #[test]
    fn test_count_lines() {
        let text = "line 1\nline 2\nline 3";
        let analyzer = TextAnalyzer::new(text);
        assert_eq!(count_lines(&analyzer), 3);
    }

    #[test]
    fn test_extract_words() {
        let text = "hello world rust";
        let words = extract_words(text);
        assert_eq!(words, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_analyzer_lifetime() {
        let text = String::from("test text");
        let analyzer = TextAnalyzer::new(&text);
        let first = analyzer.first_sentence();
        assert_eq!(first, "test text");
    }
}
