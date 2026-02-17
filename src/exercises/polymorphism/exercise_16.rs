//! Exercise 16: Returning Trait Objects - Return Box<dyn Trait> from functions
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Return trait objects from functions
//! - Choose between different implementations at runtime
//! - Understand when to use trait objects vs impl Trait

pub trait Notifier {
    fn send(&self, message: &str) -> String;
}

pub struct EmailNotifier {
    pub email: String,
}

impl Notifier for EmailNotifier {
    fn send(&self, message: &str) -> String {
        todo!("Implement send")
    }
}

pub struct SmsNotifier {
    pub phone: String,
}

impl Notifier for SmsNotifier {
    fn send(&self, message: &str) -> String {
        todo!("Implement send")
    }
}

pub struct PushNotifier {
    pub device_id: String,
}

impl Notifier for PushNotifier {
    fn send(&self, message: &str) -> String {
        todo!("Implement send")
    }
}

/// Create a notifier based on preference
pub fn create_notifier(preference: &str, target: String) -> Box<dyn Notifier> {
    todo!("Implement create_notifier")
}

/// Get default notifier
pub fn default_notifier() -> Box<dyn Notifier> {
    todo!("Implement default_notifier")
}

/// Create multiple notifiers
pub fn create_all_notifiers(target: String) -> Vec<Box<dyn Notifier>> {
    todo!("Implement create_all_notifiers")
}

/// Send via all notifiers
pub fn broadcast(message: &str, notifiers: &[Box<dyn Notifier>]) -> Vec<String> {
    todo!("Implement broadcast")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_notifier() {
        let notifier = EmailNotifier {
            email: "test@example.com".to_string(),
        };
        let result = notifier.send("Hello");
        assert!(result.contains("test@example.com"));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_sms_notifier() {
        let notifier = SmsNotifier {
            phone: "555-1234".to_string(),
        };
        let result = notifier.send("Test");
        assert!(result.contains("555-1234"));
    }

    #[test]
    fn test_create_email_notifier() {
        let notifier = create_notifier("email", "user@test.com".to_string());
        let result = notifier.send("Message");
        assert!(result.contains("user@test.com"));
    }

    #[test]
    fn test_create_sms_notifier() {
        let notifier = create_notifier("sms", "555-0000".to_string());
        let result = notifier.send("SMS Test");
        assert!(result.contains("555-0000"));
    }

    #[test]
    fn test_create_push_notifier() {
        let notifier = create_notifier("push", "device123".to_string());
        let result = notifier.send("Push Test");
        assert!(result.contains("device123"));
    }

    #[test]
    fn test_default_notifier() {
        let notifier = default_notifier();
        let result = notifier.send("Default");
        assert!(result.contains("default@example.com"));
    }

    #[test]
    fn test_create_all_notifiers() {
        let notifiers = create_all_notifiers("target".to_string());
        assert_eq!(notifiers.len(), 3);
    }

    #[test]
    fn test_broadcast() {
        let notifiers = create_all_notifiers("broadcast".to_string());
        let results = broadcast("Important", &notifiers);
        
        assert_eq!(results.len(), 3);
        assert!(results[0].contains("Email"));
        assert!(results[1].contains("SMS"));
        assert!(results[2].contains("Push"));
    }

    #[test]
    fn test_mixed_notifiers() {
        let notifiers: Vec<Box<dyn Notifier>> = vec![
            create_notifier("email", "a@test.com".to_string()),
            create_notifier("sms", "555-1111".to_string()),
            default_notifier(),
        ];
        
        let results = broadcast("Test", &notifiers);
        assert_eq!(results.len(), 3);
    }
}
