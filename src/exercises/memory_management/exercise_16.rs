//! Exercise 16: Collection Borrowing - Working with borrowed collections
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Borrow from collections safely
//! - Return references to collection elements
//! - Manage collection lifetimes

use std::collections::HashMap;

/// Find a value in a HashMap.
pub fn find_in_map<'a>(map: &'a HashMap<String, i32>, key: &str) -> Option<&'a i32> {
    todo!("Implement find_in_map")
}

/// Get all values greater than threshold.
pub fn values_greater_than<'a>(map: &'a HashMap<String, i32>, threshold: i32) -> Vec<&'a i32> {
    todo!("Implement values_greater_than")
}

/// Find key with maximum value.
pub fn key_with_max_value<'a>(map: &'a HashMap<String, i32>) -> Option<&'a String> {
    todo!("Implement key_with_max_value")
}

/// Get references to keys matching a predicate.
pub fn filter_keys<'a, F>(map: &'a HashMap<String, i32>, predicate: F) -> Vec<&'a String>
where
    F: Fn(&String) -> bool,
 {
    todo!("Get references to keys matching a predicate.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_map() -> HashMap<String, i32> {
        let mut map = HashMap::new();
        map.insert(String::from("apple"), 5);
        map.insert(String::from("banana"), 3);
        map.insert(String::from("cherry"), 8);
        map
    }

    #[test]
    fn test_find_in_map() {
        let map = create_test_map();
        assert_eq!(find_in_map(&map, "apple"), Some(&5));
        assert_eq!(find_in_map(&map, "grape"), None);
    }

    #[test]
    fn test_values_greater_than() {
        let map = create_test_map();
        let values = values_greater_than(&map, 4);
        assert_eq!(values.len(), 2);
        assert!(values.contains(&&5));
        assert!(values.contains(&&8));
    }

    #[test]
    fn test_key_with_max_value() {
        let map = create_test_map();
        let key = key_with_max_value(&map);
        assert_eq!(key, Some(&String::from("cherry")));
    }

    #[test]
    fn test_filter_keys() {
        let map = create_test_map();
        let keys = filter_keys(&map, |k| k.starts_with('b'));
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "banana");
    }

    #[test]
    fn test_combined_operations() {
        let map = create_test_map();
        let max_key = key_with_max_value(&map).unwrap();
        let max_value = find_in_map(&map, max_key).unwrap();
        assert_eq!(*max_value, 8);
    }
}
