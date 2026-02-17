//! Exercise 01: Basic Enum - Days of the Week
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Define a basic enum with simple variants
//! - Understand enum naming conventions
//! - Work with enum variants without data

/// Enum representing days of the week
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Returns true if the given day is a weekend day
pub fn is_weekend(day: DayOfWeek) -> bool {
    todo!("Implement is_weekend")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekend_days() {
        assert!(is_weekend(DayOfWeek::Saturday));
        assert!(is_weekend(DayOfWeek::Sunday));
    }

    #[test]
    fn test_weekday_days() {
        assert!(!is_weekend(DayOfWeek::Monday));
        assert!(!is_weekend(DayOfWeek::Tuesday));
        assert!(!is_weekend(DayOfWeek::Wednesday));
        assert!(!is_weekend(DayOfWeek::Thursday));
        assert!(!is_weekend(DayOfWeek::Friday));
    }
}
