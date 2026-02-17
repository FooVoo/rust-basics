//! Exercise 08: Enum Conversion - From/Into Traits
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Implement From trait for enum conversion
//! - Convert between enums and other types
//! - Understand type conversion patterns

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value % 3 {
            0 => Color::Red,
            1 => Color::Green,
            _ => Color::Blue,
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
            Color::Red => 0,
            Color::Green => 1,
            Color::Blue => 2,
        }
    }
}

/// Converts a string to a Color (case-insensitive)
pub fn parse_color(s: &str) -> Option<Color> {
    match s.to_lowercase().as_str() {
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "blue" => Some(Color::Blue),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(Color::from(0), Color::Red);
        assert_eq!(Color::from(1), Color::Green);
        assert_eq!(Color::from(2), Color::Blue);
        assert_eq!(Color::from(3), Color::Red);
        assert_eq!(Color::from(4), Color::Green);
    }

    #[test]
    fn test_to_u8() {
        assert_eq!(u8::from(Color::Red), 0);
        assert_eq!(u8::from(Color::Green), 1);
        assert_eq!(u8::from(Color::Blue), 2);
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("red"), Some(Color::Red));
        assert_eq!(parse_color("RED"), Some(Color::Red));
        assert_eq!(parse_color("green"), Some(Color::Green));
        assert_eq!(parse_color("blue"), Some(Color::Blue));
        assert_eq!(parse_color("yellow"), None);
    }
}
