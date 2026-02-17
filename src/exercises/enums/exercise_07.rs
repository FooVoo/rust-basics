//! Exercise 07: Wildcard Patterns - Using _ in Match
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use wildcard patterns in match expressions
//! - Understand catch-all patterns
//! - Group multiple patterns with |

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Returns true if the priority requires immediate attention (High or Critical)
pub fn needs_immediate_attention(priority: Priority) -> bool {
    matches!(priority, Priority::High | Priority::Critical)
}

/// Returns a numeric value for the priority
pub fn priority_value(priority: Priority) -> u8 {
    match priority {
        Priority::Critical => 4,
        Priority::High => 3,
        Priority::Medium => 2,
        Priority::Low => 1,
    }
}

/// Returns true if priority is not Low
pub fn is_important(priority: Priority) -> bool {
    match priority {
        Priority::Low => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needs_immediate_attention() {
        assert!(needs_immediate_attention(Priority::Critical));
        assert!(needs_immediate_attention(Priority::High));
        assert!(!needs_immediate_attention(Priority::Medium));
        assert!(!needs_immediate_attention(Priority::Low));
    }

    #[test]
    fn test_priority_value() {
        assert_eq!(priority_value(Priority::Low), 1);
        assert_eq!(priority_value(Priority::Medium), 2);
        assert_eq!(priority_value(Priority::High), 3);
        assert_eq!(priority_value(Priority::Critical), 4);
    }

    #[test]
    fn test_is_important() {
        assert!(!is_important(Priority::Low));
        assert!(is_important(Priority::Medium));
        assert!(is_important(Priority::High));
        assert!(is_important(Priority::Critical));
    }
}
