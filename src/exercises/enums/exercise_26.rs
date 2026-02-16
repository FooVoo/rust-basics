//! Exercise 26: Trait Implementations for Enums - Iterator and Display
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement standard library traits for enums
//! - Create custom Display implementations
//! - Work with iterator adapters on enums

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
        }
    }
}

/// Converts a vector of tokens to a string
pub fn tokens_to_string(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Filters only number tokens
pub fn filter_numbers(tokens: Vec<Token>) -> Vec<i32> {
    tokens
        .into_iter()
        .filter_map(|t| match t {
            Token::Number(n) => Some(n),
            _ => None,
        })
        .collect()
}

/// Counts the number of operator tokens
pub fn count_operators(tokens: &[Token]) -> usize {
    tokens
        .iter()
        .filter(|t| !matches!(t, Token::Number(_)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Token::Number(42).to_string(), "42");
        assert_eq!(Token::Plus.to_string(), "+");
        assert_eq!(Token::Minus.to_string(), "-");
        assert_eq!(Token::Multiply.to_string(), "*");
        assert_eq!(Token::Divide.to_string(), "/");
    }

    #[test]
    fn test_tokens_to_string() {
        let tokens = vec![
            Token::Number(10),
            Token::Plus,
            Token::Number(5),
            Token::Multiply,
            Token::Number(2),
        ];
        assert_eq!(tokens_to_string(&tokens), "10 + 5 * 2");
    }

    #[test]
    fn test_filter_numbers() {
        let tokens = vec![
            Token::Number(10),
            Token::Plus,
            Token::Number(5),
            Token::Multiply,
            Token::Number(2),
        ];
        assert_eq!(filter_numbers(tokens), vec![10, 5, 2]);
    }

    #[test]
    fn test_count_operators() {
        let tokens = vec![
            Token::Number(10),
            Token::Plus,
            Token::Number(5),
            Token::Multiply,
            Token::Number(2),
        ];
        assert_eq!(count_operators(&tokens), 2);
    }
}
