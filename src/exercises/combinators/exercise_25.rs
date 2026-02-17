//! Exercise 25: Combining multiple Results - collect pattern
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Combine multiple Results
//! - Handle all-or-nothing scenarios
//! - Use collect with Result

/// Parse all strings, fail if any fails.
pub fn parse_all_or_fail(strings: &[&str]) -> Result<Vec<i32>, String> {
    todo!("Implement parse_all_or_fail")
}

/// Process all items, collect successes and failures.
pub fn partition_results<T, E>(results: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    todo!("Implement partition_results")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_or_fail_success() {
        assert_eq!(parse_all_or_fail(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
        assert_eq!(parse_all_or_fail(&["10", "20"]), Ok(vec![10, 20]));
    }

    #[test]
    fn test_parse_all_or_fail_failure() {
        assert!(parse_all_or_fail(&["1", "abc", "3"]).is_err());
        assert!(parse_all_or_fail(&["abc"]).is_err());
    }

    #[test]
    fn test_partition_results() {
        let results = vec![Ok(1), Err("e1"), Ok(2), Err("e2"), Ok(3)];
        let (successes, failures) = partition_results(results);
        assert_eq!(successes, vec![1, 2, 3]);
        assert_eq!(failures, vec!["e1", "e2"]);
    }
}
