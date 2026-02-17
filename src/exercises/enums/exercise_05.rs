//! Exercise 05: Simple Enum with Data - Tuple Variants
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Define enum variants with tuple data
//! - Pattern match to extract data from variants
//! - Work with different variant types

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Echo(String),
    Move(i32, i32),
}

/// Returns the message type as a string
pub fn message_type(msg: &Message) -> &'static str {
    todo!("Implement message_type")
}

/// Extracts the echo message if present, otherwise returns None
pub fn get_echo_text(msg: &Message) -> Option<&str> {
    todo!("Implement get_echo_text")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type() {
        assert_eq!(message_type(&Message::Quit), "quit");
        assert_eq!(message_type(&Message::Echo("hello".to_string())), "echo");
        assert_eq!(message_type(&Message::Move(10, 20)), "move");
    }

    #[test]
    fn test_get_echo_text() {
        assert_eq!(
            get_echo_text(&Message::Echo("hello".to_string())),
            Some("hello")
        );
        assert_eq!(get_echo_text(&Message::Quit), None);
        assert_eq!(get_echo_text(&Message::Move(10, 20)), None);
    }
}
