//! Exercise 26: Collecting Results and Options
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use collect() with Result and Option
//! - Understand early termination in collections
//! - Handle vectors of Results

/// Parse all strings, failing if any parse fails.
pub fn parse_all(strings: &[&str]) -> Result<Vec<i32>, String> {
    strings
        .iter()
        .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .collect()
}

/// Filter and parse, collecting valid results.
pub fn parse_valid(strings: &[&str]) -> Vec<i32> {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

/// Try to parse all, return first error if any.
pub fn try_parse_all(
    strings: Vec<&str>,
) -> Result<Vec<i32>, std::num::ParseIntError> {
    strings.iter().map(|s| s.parse::<i32>()).collect()
}

/// Partition into successes and failures.
pub fn partition_results(
    strings: &[&str],
) -> (Vec<i32>, Vec<String>) {
    let (oks, errs): (Vec<_>, Vec<_>) = strings
        .iter()
        .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
        .partition(Result::is_ok);

    let successes = oks.into_iter().map(Result::unwrap).collect();
    let failures = errs.into_iter().map(Result::unwrap_err).collect();

    (successes, failures)
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
