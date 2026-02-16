//! Exercise 26: Generic Parser Combinator - Build a generic parser
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create parser combinators with generics
//! - Chain generic operations
//! - Work with complex generic function types

/// A generic parser result.
pub type ParseResult<'a, T> = Result<(T, &'a str), String>;

/// A generic parser trait.
pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T>;
}

/// A parser that always succeeds with a value.
pub struct Pure<T> {
    value: T,
}

impl<T: Clone> Pure<T> {
    pub fn new(value: T) -> Self {
        Pure { value }
    }
}

impl<'a, T: Clone> Parser<'a, T> for Pure<T> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, T> {
        Ok((self.value.clone(), input))
    }
}

/// A parser that matches a specific character.
pub struct CharParser {
    expected: char,
}

impl CharParser {
    pub fn new(expected: char) -> Self {
        CharParser { expected }
    }
}

impl<'a> Parser<'a, char> for CharParser {
    fn parse(&self, input: &'a str) -> ParseResult<'a, char> {
        let mut chars = input.chars();
        match chars.next() {
            Some(ch) if ch == self.expected => {
                Ok((ch, &input[ch.len_utf8()..]))
            }
            Some(ch) => Err(format!("Expected '{}', found '{}'", self.expected, ch)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

/// A parser that maps the result of another parser.
pub struct Map<P, F, A, B>
where
    P: for<'a> Parser<'a, A>,
    F: Fn(A) -> B,
{
    parser: P,
    func: F,
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<P, F, A, B> Map<P, F, A, B>
where
    P: for<'a> Parser<'a, A>,
    F: Fn(A) -> B,
{
    pub fn new(parser: P, func: F) -> Self {
        Map {
            parser,
            func,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, P, F, A, B> Parser<'a, B> for Map<P, F, A, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, B> {
        match self.parser.parse(input) {
            Ok((value, rest)) => Ok(((self.func)(value), rest)),
            Err(e) => Err(e),
        }
    }
}

/// Parses a digit character.
pub struct DigitParser;

impl<'a> Parser<'a, u32> for DigitParser {
    fn parse(&self, input: &'a str) -> ParseResult<'a, u32> {
        let mut chars = input.chars();
        match chars.next() {
            Some(ch) if ch.is_ascii_digit() => {
                let digit = ch.to_digit(10).unwrap();
                Ok((digit, &input[1..]))
            }
            Some(ch) => Err(format!("Expected digit, found '{}'", ch)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

/// Parses zero or more items.
pub fn parse_many<'a, T, P>(parser: &P, input: &'a str) -> ParseResult<'a, Vec<T>>
where
    P: Parser<'a, T>,
{
    let mut results = Vec::new();
    let mut remaining = input;

    loop {
        match parser.parse(remaining) {
            Ok((value, rest)) => {
                results.push(value);
                remaining = rest;
            }
            Err(_) => break,
        }
    }

    Ok((results, remaining))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_parser() {
        let parser = Pure::new(42);
        let result = parser.parse("hello");
        assert_eq!(result.unwrap(), (42, "hello"));
    }

    #[test]
    fn test_char_parser_success() {
        let parser = CharParser::new('a');
        let result = parser.parse("abc");
        assert_eq!(result.unwrap(), ('a', "bc"));
    }

    #[test]
    fn test_char_parser_failure() {
        let parser = CharParser::new('a');
        let result = parser.parse("bbc");
        assert!(result.is_err());
    }

    #[test]
    fn test_char_parser_empty() {
        let parser = CharParser::new('a');
        let result = parser.parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_map_parser() {
        let char_parser = CharParser::new('5');
        let digit_parser = Map::new(char_parser, |c| c.to_digit(10).unwrap());
        let result = digit_parser.parse("5abc");
        assert_eq!(result.unwrap(), (5, "abc"));
    }

    #[test]
    fn test_digit_parser() {
        let parser = DigitParser;
        let result = parser.parse("5abc");
        assert_eq!(result.unwrap(), (5, "abc"));
    }

    #[test]
    fn test_digit_parser_non_digit() {
        let parser = DigitParser;
        let result = parser.parse("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_many() {
        let parser = DigitParser;
        let result = parse_many(&parser, "123abc");
        let (digits, rest) = result.unwrap();
        assert_eq!(digits, vec![1, 2, 3]);
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_parse_many_empty() {
        let parser = DigitParser;
        let result = parse_many(&parser, "abc");
        let (digits, rest) = result.unwrap();
        assert_eq!(digits.len(), 0);
        assert_eq!(rest, "abc");
    }
}
