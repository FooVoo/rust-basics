//! Exercise 21: Custom Option combinator - tap
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Create custom combinators
//! - Understand side effects in chains
//! - Build reusable patterns

/// Custom tap combinator that allows side effects without consuming the value.
pub trait OptionTap<T> {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&T);
}

impl<T> OptionTap<T> for Option<T> {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let Some(ref value) = self {
            f(value);
        }
        self
    }
}

/// Use tap to log values in a chain.
pub fn process_with_logging(value: Option<i32>) -> Option<i32> {
    value
        .tap(|x| println!("Before: {}", x))
        .map(|x| x * 2)
        .tap(|x| println!("After: {}", x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tap_preserves_value() {
        let result = Some(5).tap(|x| assert_eq!(*x, 5));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_tap_with_none() {
        let result: Option<i32> = None.tap(|_| panic!("Should not be called"));
        assert_eq!(result, None);
    }

    #[test]
    fn test_process_with_logging() {
        assert_eq!(process_with_logging(Some(5)), Some(10));
        assert_eq!(process_with_logging(None), None);
    }
}
