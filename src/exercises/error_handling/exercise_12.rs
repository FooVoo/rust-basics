//! Exercise 12: Email Validation - Complex string validation
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement complex validation logic
//! - Use multiple validation rules
//! - Provide specific error messages

#[derive(Debug, PartialEq)]
pub enum EmailError {
    Empty,
    NoAtSign,
    MultipleAtSigns,
    NoUsername,
    NoDomain,
    InvalidDomain,
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EmailError::Empty => write!(f, "Email cannot be empty"),
            EmailError::NoAtSign => write!(f, "Email must contain @ sign"),
            EmailError::MultipleAtSigns => write!(f, "Email cannot contain multiple @ signs"),
            EmailError::NoUsername => write!(f, "Email must have a username before @"),
            EmailError::NoDomain => write!(f, "Email must have a domain after @"),
            EmailError::InvalidDomain => write!(f, "Domain must contain at least one dot"),
        }
    }
}

/// Validate an email address.
/// Must have format: username@domain.tld
pub fn validate_email(email: &str) -> Result<String, EmailError> {
    if email.is_empty() {
        return Err(EmailError::Empty);
    }
    
    let at_count = email.matches('@').count();
    if at_count == 0 {
        return Err(EmailError::NoAtSign);
    }
    if at_count > 1 {
        return Err(EmailError::MultipleAtSigns);
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    let username = parts[0];
    let domain = parts[1];
    
    if username.is_empty() {
        return Err(EmailError::NoUsername);
    }
    
    if domain.is_empty() {
        return Err(EmailError::NoDomain);
    }
    
    if !domain.contains('.') {
        return Err(EmailError::InvalidDomain);
    }
    
    Ok(email.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        assert_eq!(validate_email("user@example.com"), Ok("user@example.com".to_string()));
        assert_eq!(validate_email("admin@site.org"), Ok("admin@site.org".to_string()));
        assert_eq!(validate_email("test.user@domain.co.uk"), Ok("test.user@domain.co.uk".to_string()));
    }

    #[test]
    fn test_validate_email_empty() {
        assert_eq!(validate_email(""), Err(EmailError::Empty));
    }

    #[test]
    fn test_validate_email_no_at() {
        assert_eq!(validate_email("userexample.com"), Err(EmailError::NoAtSign));
    }

    #[test]
    fn test_validate_email_multiple_at() {
        assert_eq!(validate_email("user@@example.com"), Err(EmailError::MultipleAtSigns));
        assert_eq!(validate_email("user@domain@example.com"), Err(EmailError::MultipleAtSigns));
    }

    #[test]
    fn test_validate_email_no_username() {
        assert_eq!(validate_email("@example.com"), Err(EmailError::NoUsername));
    }

    #[test]
    fn test_validate_email_no_domain() {
        assert_eq!(validate_email("user@"), Err(EmailError::NoDomain));
    }

    #[test]
    fn test_validate_email_invalid_domain() {
        assert_eq!(validate_email("user@example"), Err(EmailError::InvalidDomain));
    }
}
