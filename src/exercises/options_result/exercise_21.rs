//! Exercise 21: Option transpose
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use transpose() to swap Option and Result
//! - Understand Option<Result<T, E>> vs Result<Option<T>, E>
//! - Handle nested type conversions

/// Convert Option<Result<T, E>> to Result<Option<T>, E>.
pub fn transpose_option_result<T, E>(
    opt: Option<Result<T, E>>,
) -> Result<Option<T>, E> {
    opt.transpose()
}

/// Parse optional strings, transposing the result.
pub fn parse_optional(opt: Option<&str>) -> Result<Option<i32>, std::num::ParseIntError> {
    opt.map(|s| s.parse::<i32>()).transpose()
}

/// Process a vector of optional parse results.
pub fn parse_all_or_none(
    strings: Vec<Option<&str>>,
) -> Result<Vec<Option<i32>>, std::num::ParseIntError> {
    strings
        .into_iter()
        .map(|opt| opt.map(|s| s.parse::<i32>()).transpose())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_option_result() {
        assert_eq!(transpose_option_result(Some(Ok::<i32, String>(42))), Ok(Some(42)));
        assert_eq!(transpose_option_result(None::<Result<i32, String>>), Ok(None));
        assert!(transpose_option_result(Some(Err::<i32, &str>("error"))).is_err());
    }

    #[test]
    fn test_parse_optional() {
        assert_eq!(parse_optional(Some("42")), Ok(Some(42)));
        assert_eq!(parse_optional(None), Ok(None));
        assert!(parse_optional(Some("abc")).is_err());
    }

    #[test]
    fn test_parse_all_or_none() {
        assert_eq!(
            parse_all_or_none(vec![Some("1"), None, Some("3")]),
            Ok(vec![Some(1), None, Some(3)])
        );
        assert!(parse_all_or_none(vec![Some("abc")]).is_err());
    }
}
