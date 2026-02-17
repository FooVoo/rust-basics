//! Exercise 25: Nested Result Handling - Work with deeply nested Results
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Handle nested Result types
//! - Flatten nested errors
//! - Use combinators effectively

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum LookupError {
    KeyNotFound(String),
    InvalidValue(String),
}

impl std::fmt::Display for LookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!("Implement fmt")
    }
}

impl std::error::Error for LookupError {}

/// Look up nested values in a map structure.
pub fn nested_lookup(
    data: &HashMap<String, HashMap<String, String>>,
    outer_key: &str,
    inner_key: &str,
) -> Result<String, LookupError>  {
    todo!("Look up nested values in a map structure.")
}

/// Parse a nested value as an integer.
pub fn nested_lookup_and_parse(
    data: &HashMap<String, HashMap<String, String>>,
    outer_key: &str,
    inner_key: &str,
) -> Result<i32, LookupError>  {
    todo!("Parse a nested value as an integer.")
}

/// Get multiple nested values, collecting all errors.
pub fn multi_nested_lookup(
    data: &HashMap<String, HashMap<String, String>>,
    queries: &[(&str, &str)],
) -> Result<Vec<String>, Vec<LookupError>>  {
    todo!("Get multiple nested values, collecting all errors.")
}

/// Chain multiple lookups together.
pub fn chain_lookups(
    data: &HashMap<String, HashMap<String, String>>,
    outer1: &str,
    inner1: &str,
    outer2: &str,
    inner2: &str,
) -> Result<(String, String), LookupError>  {
    todo!("Chain multiple lookups together.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_data() -> HashMap<String, HashMap<String, String>> {
        let mut data = HashMap::new();
        
        let mut user = HashMap::new();
        user.insert("name".to_string(), "Alice".to_string());
        user.insert("age".to_string(), "30".to_string());
        data.insert("user".to_string(), user);
        
        let mut settings = HashMap::new();
        settings.insert("theme".to_string(), "dark".to_string());
        settings.insert("timeout".to_string(), "60".to_string());
        data.insert("settings".to_string(), settings);
        
        data
    }

    #[test]
    fn test_nested_lookup_success() {
        let data = create_test_data();
        let result = nested_lookup(&data, "user", "name");
        assert_eq!(result, Ok("Alice".to_string()));
    }

    #[test]
    fn test_nested_lookup_outer_key_not_found() {
        let data = create_test_data();
        let result = nested_lookup(&data, "admin", "name");
        assert_eq!(result, Err(LookupError::KeyNotFound("admin".to_string())));
    }

    #[test]
    fn test_nested_lookup_inner_key_not_found() {
        let data = create_test_data();
        let result = nested_lookup(&data, "user", "email");
        assert_eq!(result, Err(LookupError::KeyNotFound("email".to_string())));
    }

    #[test]
    fn test_nested_lookup_and_parse_success() {
        let data = create_test_data();
        let result = nested_lookup_and_parse(&data, "user", "age");
        assert_eq!(result, Ok(30));
    }

    #[test]
    fn test_nested_lookup_and_parse_invalid() {
        let data = create_test_data();
        let result = nested_lookup_and_parse(&data, "user", "name");
        assert!(matches!(result, Err(LookupError::InvalidValue(_))));
    }

    #[test]
    fn test_nested_lookup_and_parse_not_found() {
        let data = create_test_data();
        let result = nested_lookup_and_parse(&data, "user", "salary");
        assert!(matches!(result, Err(LookupError::KeyNotFound(_))));
    }

    #[test]
    fn test_multi_nested_lookup_all_success() {
        let data = create_test_data();
        let queries = &[("user", "name"), ("user", "age"), ("settings", "theme")];
        let result = multi_nested_lookup(&data, queries);
        
        assert!(result.is_ok());
        let values = result.unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], "Alice");
    }

    #[test]
    fn test_multi_nested_lookup_some_errors() {
        let data = create_test_data();
        let queries = &[("user", "name"), ("user", "email"), ("admin", "name")];
        let result = multi_nested_lookup(&data, queries);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_chain_lookups_success() {
        let data = create_test_data();
        let result = chain_lookups(&data, "user", "name", "settings", "theme");
        
        assert_eq!(result, Ok(("Alice".to_string(), "dark".to_string())));
    }

    #[test]
    fn test_chain_lookups_first_fails() {
        let data = create_test_data();
        let result = chain_lookups(&data, "admin", "name", "settings", "theme");
        
        assert!(result.is_err());
    }

    #[test]
    fn test_chain_lookups_second_fails() {
        let data = create_test_data();
        let result = chain_lookups(&data, "user", "name", "settings", "invalid");
        
        assert!(result.is_err());
    }
}
