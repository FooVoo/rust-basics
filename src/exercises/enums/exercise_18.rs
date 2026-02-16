//! Exercise 18: Enum Comparison - Implementing Ordering
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement PartialOrd and Ord for enums
//! - Define custom ordering logic
//! - Compare enum variants

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_rank = match self {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
            Priority::Critical => 3,
        };
        let other_rank = match other {
            Priority::Low => 0,
            Priority::Medium => 1,
            Priority::High => 2,
            Priority::Critical => 3,
        };
        self_rank.cmp(&other_rank)
    }
}

/// Returns the higher priority
pub fn max_priority(a: Priority, b: Priority) -> Priority {
    if a > b { a } else { b }
}

/// Sorts priorities in descending order (highest first)
pub fn sort_by_priority(mut priorities: Vec<Priority>) -> Vec<Priority> {
    priorities.sort_by(|a, b| b.cmp(a));
    priorities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        assert!(Priority::Low < Priority::Critical);
    }

    #[test]
    fn test_max_priority() {
        assert_eq!(max_priority(Priority::Low, Priority::High), Priority::High);
        assert_eq!(max_priority(Priority::Critical, Priority::Medium), Priority::Critical);
        assert_eq!(max_priority(Priority::Low, Priority::Low), Priority::Low);
    }

    #[test]
    fn test_sort_by_priority() {
        let priorities = vec![Priority::Low, Priority::Critical, Priority::Medium, Priority::High];
        let sorted = sort_by_priority(priorities);
        assert_eq!(
            sorted,
            vec![Priority::Critical, Priority::High, Priority::Medium, Priority::Low]
        );
    }
}
