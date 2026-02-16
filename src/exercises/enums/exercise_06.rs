//! Exercise 06: Enum in Collections - Working with Vectors
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Store enums in collections
//! - Iterate over enum values
//! - Filter and count enum variants

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Counts how many items have the Completed status
pub fn count_completed(statuses: &[Status]) -> usize {
    statuses
        .iter()
        .filter(|s| matches!(s, Status::Completed))
        .count()
}

/// Returns true if all statuses are Completed
pub fn all_completed(statuses: &[Status]) -> bool {
    statuses.iter().all(|s| matches!(s, Status::Completed))
}

/// Returns true if any status is Failed
pub fn has_failure(statuses: &[Status]) -> bool {
    statuses.iter().any(|s| matches!(s, Status::Failed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_completed() {
        let statuses = vec![
            Status::Completed,
            Status::Pending,
            Status::Completed,
            Status::Failed,
        ];
        assert_eq!(count_completed(&statuses), 2);

        let all_completed = vec![Status::Completed, Status::Completed];
        assert_eq!(count_completed(&all_completed), 2);

        let none_completed = vec![Status::Pending, Status::Failed];
        assert_eq!(count_completed(&none_completed), 0);
    }

    #[test]
    fn test_all_completed() {
        assert!(all_completed(&[Status::Completed, Status::Completed]));
        assert!(!all_completed(&[Status::Completed, Status::Pending]));
        assert!(all_completed(&[]));
    }

    #[test]
    fn test_has_failure() {
        assert!(has_failure(&[Status::Completed, Status::Failed]));
        assert!(!has_failure(&[Status::Completed, Status::Pending]));
        assert!(!has_failure(&[]));
    }
}
