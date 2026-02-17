//! Exercise 30: Building a complete combinator library
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Design a complete combinator system
//! - Implement a mini parsing combinator library
//! - Master advanced functional composition

/// Parser combinator that transforms input.
pub struct Parser<I, O> {
    parse: Box<dyn Fn(I) -> Option<(O, I)>>,
}

impl<I, O> Parser<I, O> {
    /// Create a new parser.
    pub fn new<F>(parse: F) -> Self
    where
        F: Fn(I) -> Option<(O, I)> + 'static,
    {
        Parser {
            parse: Box::new(parse),
        }
    }

    /// Run the parser on input.
    pub fn parse(&self, input: I) -> Option<(O, I)> {
        (self.parse)(input)
    }

    /// Map the output of the parser.
    pub fn map<O2, F>(self, f: F) -> Parser<I, O2>
    where
        F: Fn(O) -> O2 + 'static,
        I: 'static,
        O: 'static,
    {
        Parser::new(move |input| self.parse(input).map(|(output, rest)| (f(output), rest)))
    }

    /// Chain two parsers together.
    pub fn and_then<O2, F>(self, f: F) -> Parser<I, O2>
    where
        F: Fn(O) -> Parser<I, O2> + 'static,
        I: Clone + 'static,
        O: 'static,
    {
        Parser::new(move |input: I| {
            self.parse(input.clone())
                .and_then(|(output, rest)| f(output).parse(rest))
        })
    }

    /// Try this parser, or try another if it fails.
    pub fn or(self, other: Parser<I, O>) -> Parser<I, O>
    where
        I: Clone + 'static,
        O: 'static,
    {
        Parser::new(move |input: I| {
            self.parse(input.clone()).or_else(|| other.parse(input))
        })
    }
}

/// Parser for a single character.
pub fn char_parser(expected: char) -> Parser<String, char> {
    Parser::new(move |input: String| {
        let mut chars = input.chars();
        match chars.next() {
            Some(ch) if ch == expected => Some((ch, chars.collect())),
            _ => None,
        }
    })
}

/// Parser for any digit.
pub fn digit_parser() -> Parser<String, char> {
    Parser::new(|input: String| {
        let mut chars = input.chars();
        match chars.next() {
            Some(ch) if ch.is_ascii_digit() => Some((ch, chars.collect())),
            _ => None,
        }
    })
}

/// Parse a digit and convert to number.
pub fn digit_value_parser() -> Parser<String, i32> {
    digit_parser().map(|ch| ch.to_digit(10).unwrap() as i32)
}

/// Parse two digits and add them.
pub fn two_digit_sum_parser() -> Parser<String, i32> {
    let parser = digit_value_parser();
    Parser::new(move |input: String| {
        parser.parse(input.clone()).and_then(|(first, rest1)| {
            digit_value_parser()
                .parse(rest1)
                .map(|(second, rest2)| (first + second, rest2))
        })
    })
}

/// Parse a specific string literal.
pub fn string_parser(expected: &'static str) -> Parser<String, String> {
    Parser::new(move |input: String| {
        if input.starts_with(expected) {
            Some((expected.to_string(), input[expected.len()..].to_string()))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_parser() {
        let parser = char_parser('a');
        assert_eq!(parser.parse("abc".to_string()), Some(('a', "bc".to_string())));
        assert_eq!(parser.parse("bac".to_string()), None);
    }

    #[test]
    fn test_digit_parser() {
        let parser = digit_parser();
        assert_eq!(parser.parse("123".to_string()), Some(('1', "23".to_string())));
        assert_eq!(parser.parse("abc".to_string()), None);
    }

    #[test]
    fn test_digit_value_parser() {
        let parser = digit_value_parser();
        assert_eq!(parser.parse("5abc".to_string()), Some((5, "abc".to_string())));
        assert_eq!(parser.parse("9".to_string()), Some((9, "".to_string())));
    }

    #[test]
    fn test_two_digit_sum() {
        let parser = two_digit_sum_parser();
        assert_eq!(parser.parse("23abc".to_string()), Some((5, "abc".to_string())));
        assert_eq!(parser.parse("99".to_string()), Some((18, "".to_string())));
        assert_eq!(parser.parse("1".to_string()), None);
    }

    #[test]
    fn test_string_parser() {
        let parser = string_parser("hello");
        assert_eq!(
            parser.parse("hello world".to_string()),
            Some(("hello".to_string(), " world".to_string()))
        );
        assert_eq!(parser.parse("hi".to_string()), None);
    }

    #[test]
    fn test_parser_or() {
        let parser = char_parser('a').or(char_parser('b'));
        assert_eq!(parser.parse("abc".to_string()), Some(('a', "bc".to_string())));
        assert_eq!(parser.parse("bac".to_string()), Some(('b', "ac".to_string())));
        assert_eq!(parser.parse("cab".to_string()), None);
    }

    #[test]
    fn test_parser_map() {
        let parser = digit_parser().map(|ch| ch.to_uppercase().next().unwrap());
        assert_eq!(parser.parse("5abc".to_string()), Some(('5', "abc".to_string())));
    }
}
