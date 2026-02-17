//! Exercise 17: Marker Traits - Create and use marker traits
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Understand marker traits
//! - Use traits for compile-time type categorization
//! - Implement conditional behavior based on markers

/// Marker trait indicating a type is serializable
pub trait Serializable {}

/// Marker trait indicating a type is cacheable
pub trait Cacheable {}

/// Marker trait indicating a type is network-safe
pub trait NetworkSafe {}

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
}

impl Serializable for User {}
impl Cacheable for User {}
impl NetworkSafe for User {}

#[derive(Clone)]
pub struct Config {
    pub settings: String,
}

impl Serializable for Config {}
impl Cacheable for Config {}

#[derive(Clone)]
pub struct SecretKey {
    pub key: String,
}

impl Serializable for SecretKey {}
// Not Cacheable or NetworkSafe

/// Only works with Serializable types
pub fn serialize<T: Serializable>(_item: &T) -> String {
    todo!("Implement serialize")
}

/// Only works with Cacheable types
pub fn cache<T: Cacheable + Clone>(item: &T) -> T {
    todo!("Implement cache")
}

/// Only works with NetworkSafe types
pub fn send_over_network<T: NetworkSafe>(_item: &T) -> String {
    todo!("Implement send_over_network")
}

/// Works with all three markers
pub fn full_process<T: Serializable + Cacheable + NetworkSafe + Clone>(item: &T) -> (String, T, String) {
    todo!("Implement full_process")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_is_serializable() {
        let user = User {
            id: 1,
            name: "Alice".to_string(),
        };
        let result = serialize(&user);
        assert!(result.contains("Serialized"));
    }

    #[test]
    fn test_user_is_cacheable() {
        let user = User {
            id: 2,
            name: "Bob".to_string(),
        };
        let cached = cache(&user);
        assert_eq!(cached.id, user.id);
    }

    #[test]
    fn test_user_is_network_safe() {
        let user = User {
            id: 3,
            name: "Charlie".to_string(),
        };
        let result = send_over_network(&user);
        assert!(result.contains("network"));
    }

    #[test]
    fn test_config_serializable_and_cacheable() {
        let config = Config {
            settings: "debug=true".to_string(),
        };
        let _serialized = serialize(&config);
        let _cached = cache(&config);
    }

    #[test]
    fn test_secret_key_only_serializable() {
        let key = SecretKey {
            key: "secret123".to_string(),
        };
        let _serialized = serialize(&key);
        // Can't cache or send over network
    }

    #[test]
    fn test_full_process_user() {
        let user = User {
            id: 4,
            name: "Diana".to_string(),
        };
        let (serialized, cached, sent) = full_process(&user);
        
        assert!(serialized.contains("Serialized"));
        assert_eq!(cached.id, user.id);
        assert!(sent.contains("network"));
    }

    #[test]
    fn test_marker_traits_compile_time() {
        // This demonstrates compile-time guarantees
        let user = User {
            id: 5,
            name: "Eve".to_string(),
        };
        
        // User has all markers, so this compiles
        let _ = full_process(&user);
        
        // Config doesn't have NetworkSafe, so full_process won't compile for it
        // let config = Config { settings: "test".to_string() };
        // let _ = full_process(&config); // Would not compile
    }

    #[test]
    fn test_selective_operations() {
        let user = User {
            id: 6,
            name: "Frank".to_string(),
        };
        let config = Config {
            settings: "prod=true".to_string(),
        };
        let key = SecretKey {
            key: "key456".to_string(),
        };
        
        // All can be serialized
        let _ = serialize(&user);
        let _ = serialize(&config);
        let _ = serialize(&key);
        
        // Only user and config can be cached
        let _ = cache(&user);
        let _ = cache(&config);
        
        // Only user can be sent over network
        let _ = send_over_network(&user);
    }
}
