//! Exercise 23: JSON Value - Complex Nested Enum
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Model complex nested data structures
//! - Work with enums containing multiple data types
//! - Implement accessors for nested data

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Returns true if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    /// Gets a value from an object by key
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(map) => map.get(key),
            _ => None,
        }
    }

    /// Gets a value from an array by index
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(vec) => vec.get(index),
            _ => None,
        }
    }

    /// Extracts a string value
    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Extracts a number value
    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Extracts a boolean value
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitives() {
        assert!(JsonValue::Null.is_null());
        assert_eq!(JsonValue::Bool(true).as_bool(), Some(true));
        assert_eq!(JsonValue::Number(42.0).as_number(), Some(42.0));
        assert_eq!(
            JsonValue::String("hello".to_string()).as_string(),
            Some("hello")
        );
    }

    #[test]
    fn test_array() {
        let arr = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        assert_eq!(arr.get_index(0).unwrap().as_number(), Some(1.0));
        assert_eq!(arr.get_index(2).unwrap().as_number(), Some(3.0));
        assert!(arr.get_index(3).is_none());
    }

    #[test]
    fn test_object() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), JsonValue::String("Alice".to_string()));
        map.insert("age".to_string(), JsonValue::Number(30.0));
        let obj = JsonValue::Object(map);

        assert_eq!(obj.get("name").unwrap().as_string(), Some("Alice"));
        assert_eq!(obj.get("age").unwrap().as_number(), Some(30.0));
        assert!(obj.get("unknown").is_none());
    }

    #[test]
    fn test_nested_structure() {
        let mut inner = HashMap::new();
        inner.insert("city".to_string(), JsonValue::String("NYC".to_string()));

        let mut outer = HashMap::new();
        outer.insert("address".to_string(), JsonValue::Object(inner));

        let obj = JsonValue::Object(outer);
        let nested = obj.get("address").and_then(|v| v.get("city"));
        assert_eq!(nested.unwrap().as_string(), Some("NYC"));
    }
}
