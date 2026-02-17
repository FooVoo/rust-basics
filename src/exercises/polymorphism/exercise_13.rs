//! Exercise 13: Multiple Trait Bounds - Combine multiple trait requirements
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use multiple trait bounds
//! - Combine traits with + syntax
//! - Understand when multiple bounds are needed

use std::fmt::Display;

pub trait Identifiable {
    fn id(&self) -> u64;
}

pub trait Timestamped {
    fn timestamp(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u64,
    pub timestamp: u64,
    pub message: String,
}

impl Identifiable for Event {
    fn id(&self) -> u64  {
        todo!("Implement id")
    }
}

impl Timestamped for Event {
    fn timestamp(&self) -> u64  {
        todo!("Implement timestamp")
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {
        todo!("Implement fmt")
    }
}

/// Function requiring both Identifiable and Timestamped
pub fn log_item<T: Identifiable + Timestamped>(item: &T) -> String  {
    todo!("Function requiring both Identifiable and Timestamped")
}

/// Function requiring all three traits
pub fn full_log<T: Identifiable + Timestamped + Display>(item: &T) -> String  {
    todo!("Function requiring all three traits")
}

/// Function with Clone bound
pub fn duplicate_and_log<T: Identifiable + Clone>(item: &T) -> (T, String)  {
    todo!("Function with Clone bound")
}

/// Using where clause for readability
pub fn complex_operation<T>(item: &T) -> String
where
    T: Identifiable + Timestamped + Display + Clone,
 {
    todo!("Using where clause for readability")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_identifiable() {
        let event = Event {
            id: 1,
            timestamp: 1000,
            message: "Test".to_string(),
        };
        assert_eq!(event.id(), 1);
    }

    #[test]
    fn test_event_timestamped() {
        let event = Event {
            id: 2,
            timestamp: 2000,
            message: "Test".to_string(),
        };
        assert_eq!(event.timestamp(), 2000);
    }

    #[test]
    fn test_event_display() {
        let event = Event {
            id: 3,
            timestamp: 3000,
            message: "Display test".to_string(),
        };
        let display = format!("{}", event);
        assert!(display.contains("Event #3"));
        assert!(display.contains("3000"));
        assert!(display.contains("Display test"));
    }

    #[test]
    fn test_log_item() {
        let event = Event {
            id: 4,
            timestamp: 4000,
            message: "Log test".to_string(),
        };
        let log = log_item(&event);
        assert!(log.contains("ID: 4"));
        assert!(log.contains("Time: 4000"));
    }

    #[test]
    fn test_full_log() {
        let event = Event {
            id: 5,
            timestamp: 5000,
            message: "Full log".to_string(),
        };
        let log = full_log(&event);
        assert!(log.contains("Event #5"));
        assert!(log.contains("ID: 5"));
        assert!(log.contains("Time: 5000"));
    }

    #[test]
    fn test_duplicate_and_log() {
        let event = Event {
            id: 6,
            timestamp: 6000,
            message: "Clone test".to_string(),
        };
        let (cloned, log) = duplicate_and_log(&event);
        
        assert_eq!(cloned.id, event.id);
        assert!(log.contains("ID: 6"));
    }

    #[test]
    fn test_complex_operation() {
        let event = Event {
            id: 7,
            timestamp: 7000,
            message: "Complex".to_string(),
        };
        let result = complex_operation(&event);
        assert!(result.contains("Processed:"));
        assert!(result.contains("Event #7"));
    }

    #[test]
    fn test_multiple_events() {
        let events = vec![
            Event { id: 1, timestamp: 1000, message: "First".to_string() },
            Event { id: 2, timestamp: 2000, message: "Second".to_string() },
            Event { id: 3, timestamp: 3000, message: "Third".to_string() },
        ];
        
        let logs: Vec<String> = events.iter().map(|e| log_item(e)).collect();
        assert_eq!(logs.len(), 3);
        assert!(logs[0].contains("ID: 1"));
        assert!(logs[2].contains("ID: 3"));
    }
}
