//! Exercise 07: Username Validation - Multi-criteria validation
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Validate strings against multiple criteria
//! - Return specific error messages for different failures
//! - Use string methods for validation

/// Validate a username.
/// Must be 3-20 characters, alphanumeric, and not empty.
pub fn validate_username(username: &str) -> Result<String, String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    
    if username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    
    if username.len() > 20 {
        return Err("Username must be at most 20 characters".to_string());
    }
    
    if !username.chars().all(|c| c.is_alphanumeric()) {
        return Err("Username must be alphanumeric".to_string());
    }
    
    Ok(username.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username_valid() {
        assert_eq!(validate_username("john"), Ok("john".to_string()));
        assert_eq!(validate_username("User123"), Ok("User123".to_string()));
        assert_eq!(validate_username("abc"), Ok("abc".to_string()));
        assert_eq!(validate_username("a1b2c3d4e5f6g7h8i9j0"), Ok("a1b2c3d4e5f6g7h8i9j0".to_string()));
    }

    #[test]
    fn test_validate_username_empty() {
        assert_eq!(
            validate_username(""),
            Err("Username cannot be empty".to_string())
        );
    }

    #[test]
    fn test_validate_username_too_short() {
        assert_eq!(
            validate_username("ab"),
            Err("Username must be at least 3 characters".to_string())
        );
    }

    #[test]
    fn test_validate_username_too_long() {
        assert_eq!(
            validate_username("thisusernameiswaytoolong"),
            Err("Username must be at most 20 characters".to_string())
        );
    }

    #[test]
    fn test_validate_username_special_chars() {
        assert_eq!(
            validate_username("user@name"),
            Err("Username must be alphanumeric".to_string())
        );
        assert_eq!(
            validate_username("user_name"),
            Err("Username must be alphanumeric".to_string())
        );
    }
}
