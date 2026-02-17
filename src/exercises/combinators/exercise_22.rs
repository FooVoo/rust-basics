//! Exercise 22: Custom Result combinator - map_both
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Transform both Ok and Err variants
//! - Build advanced combinators
//! - Handle dual transformations

/// Custom combinator to map both Ok and Err.
pub trait ResultMapBoth<T, E> {
    fn map_both<U, F, T2, E2>(self, ok_fn: T2, err_fn: E2) -> Result<U, F>
    where
        T2: FnOnce(T) -> U,
        E2: FnOnce(E) -> F;
}

impl<T, E> ResultMapBoth<T, E> for Result<T, E> {
    fn map_both<U, F, T2, E2>(self, ok_fn: T2, err_fn: E2) -> Result<U, F>
    where
        T2: FnOnce(T) -> U,
        E2: FnOnce(E) -> F,
     {
        todo!("Implement map_both")
    }
}

/// Use map_both to transform parse result.
pub fn parse_and_transform(s: &str) -> Result<String, String> {
    todo!("Implement parse_and_transform")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_both_ok() {
        let result: Result<i32, String> = Ok(5);
        let mapped = result.map_both(|x| x * 2, |e| format!("Error: {}", e));
        assert_eq!(mapped, Ok(10));
    }

    #[test]
    fn test_map_both_err() {
        let result: Result<i32, String> = Err("failed".to_string());
        let mapped = result.map_both(|x| x * 2, |e| format!("Error: {}", e));
        assert_eq!(mapped, Err("Error: failed".to_string()));
    }

    #[test]
    fn test_parse_and_transform() {
        assert_eq!(parse_and_transform("5"), Ok("Success: 10".to_string()));
        assert!(parse_and_transform("abc").is_err());
    }
}
