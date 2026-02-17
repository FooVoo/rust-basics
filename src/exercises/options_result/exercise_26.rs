//! Exercise 26: Collecting Results and Options
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use collect() with Result and Option
//! - Understand early termination in collections
//! - Handle vectors of Results

/// Parse all strings, failing if any parse fails.
pub fn parse_all(strings: &[&str]) -> Result<Vec<i32>, String> {
    todo!("Implement parse_all")
}

/// Filter and parse, collecting valid results.
pub fn parse_valid(strings: &[&str]) -> Vec<i32> {
    todo!("Implement parse_valid")
}

/// Try to parse all, return first error if any.
pub fn try_parse_all(
    strings: Vec<&str>,
) -> Result<Vec<i32>, std::num::ParseIntError>  {
    todo!("Try to parse all, return first error if any.")
}

/// Partition into successes and failures.
pub fn partition_results(
    strings: &[&str],
) -> (Vec<i32>, Vec<String>)  {
    todo!("Partition into successes and failures.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all() {
        assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
        assert!(parse_all(&["1", "abc", "3"]).is_err());
    }

    #[test]
    fn test_parse_valid() {
        assert_eq!(parse_valid(&["1", "abc", "3", "xyz"]), vec![1, 3]);
        assert_eq!(parse_valid(&["abc", "xyz"]), vec![]);
        assert_eq!(parse_valid(&["1", "2", "3"]), vec![1, 2, 3]);
    }

    #[test]
    fn test_try_parse_all() {
        assert_eq!(try_parse_all(vec!["1", "2", "3"]), Ok(vec![1, 2, 3]));
        assert!(try_parse_all(vec!["1", "abc", "3"]).is_err());
    }

    #[test]
    fn test_partition_results() {
        let (successes, failures) = partition_results(&["1", "abc", "3", "xyz"]);
        assert_eq!(successes, vec![1, 3]);
        assert_eq!(failures.len(), 2);
    }
}
