//! Exercise 24: Custom combinator - inspect
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create inspection combinators
//! - Debug chains without breaking them
//! - Build developer tools

/// Inspect combinator for Result that allows peeking at values.
pub trait ResultInspect<T, E> {
    fn inspect_ok<F>(self, f: F) -> Self
    where
        F: FnOnce(&T);

    fn inspect_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E);
}

impl<T, E> ResultInspect<T, E> for Result<T, E> {
    fn inspect_ok<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
     {
        todo!("Implement inspect_ok")
    }

    fn inspect_err<F>(self, f: F) -> Self
    where
        F: FnOnce(&E),
     {
        todo!("Implement inspect_err")
    }
}

/// Process with inspection at each step.
pub fn parse_with_inspection(s: &str) -> Result<i32, String> {
    todo!("Implement parse_with_inspection")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_ok() {
        let mut called = false;
        let result: Result<i32, String> = Ok(5);
        let output = result.inspect_ok(|x| {
            called = true;
            assert_eq!(*x, 5);
        });
        assert!(called);
        assert_eq!(output, Ok(5));
    }

    #[test]
    fn test_inspect_err() {
        let mut called = false;
        let result: Result<i32, String> = Err("error".to_string());
        let output = result.inspect_err(|e| {
            called = true;
            assert_eq!(e, "error");
        });
        assert!(called);
        assert_eq!(output, Err("error".to_string()));
    }

    #[test]
    fn test_parse_with_inspection() {
        assert_eq!(parse_with_inspection("5"), Ok(10));
        assert!(parse_with_inspection("abc").is_err());
    }
}
