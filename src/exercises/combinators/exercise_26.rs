//! Exercise 26: Custom combinator - try_map
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Build fallible map operations
//! - Combine map and and_then patterns
//! - Handle transformation failures

/// TryMap trait for Option that allows fallible mapping.
pub trait OptionTryMap<T> {
    fn try_map<U, E, F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>;
}

impl<T> OptionTryMap<T> for Option<T> {
    fn try_map<U, E, F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
     {
        todo!("Implement try_map")
    }
}

/// Parse optional string value.
pub fn try_parse_option(s: Option<&str>) -> Result<Option<i32>, String> {
    todo!("Implement try_parse_option")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_map_some_success() {
        let value = Some(5);
        let result = value.try_map(|x| Ok::<_, String>(x * 2));
        assert_eq!(result, Ok(Some(10)));
    }

    #[test]
    fn test_try_map_some_failure() {
        let value = Some(5);
        let result = value.try_map(|_| Err::<i32, _>("error".to_string()));
        assert_eq!(result, Err("error".to_string()));
    }

    #[test]
    fn test_try_map_none() {
        let value: Option<i32> = None;
        let result = value.try_map(|x| Ok::<_, String>(x * 2));
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_try_parse_option() {
        assert_eq!(try_parse_option(Some("42")), Ok(Some(42)));
        assert_eq!(try_parse_option(None), Ok(None));
        assert!(try_parse_option(Some("abc")).is_err());
    }
}
