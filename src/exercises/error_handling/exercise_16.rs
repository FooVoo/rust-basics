//! Exercise 16: JSON Parsing - Handle structured data errors
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Parse structured data with validation
//! - Handle missing fields
//! - Provide detailed error information

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonError {
    MissingField(String),
    InvalidType(String),
    InvalidValue(String),
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JsonError::MissingField(field) => write!(f, "Missing required field: {}", field),
            JsonError::InvalidType(msg) => write!(f, "Invalid type: {}", msg),
            JsonError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
        }
    }
}

impl std::error::Error for JsonError {}

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
}

/// Parse a user from a simple key-value map.
pub fn parse_user(data: &HashMap<String, String>) -> Result<User, JsonError> {
    let name = data
        .get("name")
        .ok_or_else(|| JsonError::MissingField("name".to_string()))?
        .clone();
    
    if name.is_empty() {
        return Err(JsonError::InvalidValue("name cannot be empty".to_string()));
    }
    
    let age_str = data
        .get("age")
        .ok_or_else(|| JsonError::MissingField("age".to_string()))?;
    
    let age = age_str
        .parse::<u32>()
        .map_err(|_| JsonError::InvalidType("age must be a number".to_string()))?;
    
    if age > 150 {
        return Err(JsonError::InvalidValue("age must be <= 150".to_string()));
    }
    
    let email = data
        .get("email")
        .ok_or_else(|| JsonError::MissingField("email".to_string()))?
        .clone();
    
    if !email.contains('@') {
        return Err(JsonError::InvalidValue("email must contain @".to_string()));
    }
    
    Ok(User { name, age, email })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_valid() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Alice".to_string());
        data.insert("age".to_string(), "30".to_string());
        data.insert("email".to_string(), "alice@example.com".to_string());
        
        let result = parse_user(&data);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
        assert_eq!(user.email, "alice@example.com");
    }

    #[test]
    fn test_parse_user_missing_name() {
        let mut data = HashMap::new();
        data.insert("age".to_string(), "30".to_string());
        data.insert("email".to_string(), "alice@example.com".to_string());
        
        assert_eq!(
            parse_user(&data),
            Err(JsonError::MissingField("name".to_string()))
        );
    }

    #[test]
    fn test_parse_user_missing_age() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Alice".to_string());
        data.insert("email".to_string(), "alice@example.com".to_string());
        
        assert_eq!(
            parse_user(&data),
            Err(JsonError::MissingField("age".to_string()))
        );
    }

    #[test]
    fn test_parse_user_invalid_age() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Alice".to_string());
        data.insert("age".to_string(), "abc".to_string());
        data.insert("email".to_string(), "alice@example.com".to_string());
        
        assert!(matches!(parse_user(&data), Err(JsonError::InvalidType(_))));
    }

    #[test]
    fn test_parse_user_age_too_high() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Alice".to_string());
        data.insert("age".to_string(), "200".to_string());
        data.insert("email".to_string(), "alice@example.com".to_string());
        
        assert!(matches!(parse_user(&data), Err(JsonError::InvalidValue(_))));
    }

    #[test]
    fn test_parse_user_invalid_email() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), "Alice".to_string());
        data.insert("age".to_string(), "30".to_string());
        data.insert("email".to_string(), "invalid".to_string());
        
        assert!(matches!(parse_user(&data), Err(JsonError::InvalidValue(_))));
    }

    #[test]
    fn test_parse_user_empty_name() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), "".to_string());
        data.insert("age".to_string(), "30".to_string());
        data.insert("email".to_string(), "alice@example.com".to_string());
        
        assert!(matches!(parse_user(&data), Err(JsonError::InvalidValue(_))));
    }
}
