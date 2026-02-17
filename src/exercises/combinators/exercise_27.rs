//! Exercise 27: Retry combinator pattern
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Build retry mechanisms
//! - Handle transient failures
//! - Implement robust error recovery

/// Retry a fallible operation multiple times.
pub fn retry<T, E, F>(mut operation: F, max_attempts: usize) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
 {
    todo!("Retry a fallible operation multiple times.")
}

/// Retry with exponential backoff (simulated).
pub fn retry_with_transform<T, E, F>(
    mut operation: F,
    max_attempts: usize,
    transform_error: fn(E, usize) -> E,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
 {
    todo!("Retry with exponential backoff (simulated).")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_retry_success_first_try() {
        let result = retry(|| Ok::<_, String>(42), 3);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_retry_success_after_failures() {
        let counter = Cell::new(0);
        let result = retry(
            || {
                counter.set(counter.get() + 1);
                if counter.get() < 3 {
                    Err("not yet")
                } else {
                    Ok(42)
                }
            },
            5,
        );
        assert_eq!(result, Ok(42));
        assert_eq!(counter.get(), 3);
    }

    #[test]
    fn test_retry_max_attempts() {
        let result = retry(|| Err::<i32, _>("failed"), 3);
        assert_eq!(result, Err("failed"));
    }

    #[test]
    fn test_retry_with_transform() {
        let result: Result<i32, String> = retry_with_transform(
            || Err::<i32, _>("error".to_string()),
            3,
            |e, attempts| format!("{} (attempted {} times)", e, attempts),
        );
        assert_eq!(result, Err("error (attempted 3 times)".to_string()));
    }
}
