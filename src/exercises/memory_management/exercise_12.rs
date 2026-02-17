//! Exercise 12: String Interning - Working with static and dynamic strings
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand the difference between &'static str and String
//! - Work with string literals and heap-allocated strings
//! - Convert between string types

/// Return a static string.
pub fn get_greeting() -> &'static str  {
    todo!("Return a static string.")
}

/// Choose between a static string and a reference based on condition.
pub fn choose_string<'a>(use_static: bool, dynamic: &'a str) -> &'a str  {
    todo!("Choose between a static string and a reference based on condition.")
}

/// Create a string with proper lifetime management.
pub fn format_name(first: &str, last: &str) -> String  {
    todo!("Create a string with proper lifetime management.")
}

/// Return the appropriate error message.
pub fn get_error_message(code: i32) -> &'static str  {
    todo!("Return the appropriate error message.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        let greeting = get_greeting();
        assert_eq!(greeting, "Hello, Rust!");
    }

    #[test]
    fn test_choose_string() {
        let dynamic = String::from("dynamic");
        assert_eq!(choose_string(true, &dynamic), "static string");
        assert_eq!(choose_string(false, &dynamic), "dynamic");
    }

    #[test]
    fn test_format_name() {
        let name = format_name("John", "Doe");
        assert_eq!(name, "John Doe");
    }

    #[test]
    fn test_get_error_message() {
        assert_eq!(get_error_message(404), "Not Found");
        assert_eq!(get_error_message(500), "Internal Server Error");
        assert_eq!(get_error_message(999), "Unknown Error");
    }

    #[test]
    fn test_static_lifetime() {
        let msg;
        {
            msg = get_greeting();
        }
        // msg is still valid because it's 'static
        assert_eq!(msg, "Hello, Rust!");
    }

    #[test]
    fn test_string_conversion() {
        let static_str = get_greeting();
        let owned = static_str.to_string();
        let name = format_name(&owned, "User");
        assert!(name.contains("Hello"));
    }
}
