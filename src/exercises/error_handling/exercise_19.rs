//! Exercise 19: Retry Logic - Implement retry mechanisms
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Implement retry logic
//! - Handle transient errors
//! - Track retry attempts

#[derive(Debug, PartialEq)]
pub enum RetryError<E> {
    MaxRetriesExceeded(Vec<E>),
    PermanentError(E),
}

impl<E: std::fmt::Display> std::fmt::Display for RetryError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!("Implement fmt")
    }
}

impl<E: std::fmt::Debug + std::fmt::Display> std::error::Error for RetryError<E> {}

/// Retry a function up to max_attempts times.
/// Collect all errors and return them if all attempts fail.
pub fn retry<T, E, F>(mut f: F, max_attempts: usize) -> Result<T, RetryError<E>>
where
    F: FnMut() -> Result<T, E>,
 {
    todo!("Collect all errors and return them if all attempts fail.")
}

/// Retry with a predicate to determine if error is retryable.
pub fn retry_with_predicate<T, E, F, P>(
    mut f: F,
    max_attempts: usize,
    is_retryable: P,
) -> Result<T, RetryError<E>>
where
    F: FnMut() -> Result<T, E>,
    P: Fn(&E) -> bool,
 {
    todo!("Implement retry_with_predicate")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_success_first_attempt() {
        let result = retry(|| Ok::<i32, String>(42), 3);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_retry_success_after_failures() {
        let mut attempts = 0;
        let result = retry(
            || {
                attempts += 1;
                if attempts < 3 {
                    Err("not yet")
                } else {
                    Ok(42)
                }
            },
            5,
        );
        assert_eq!(result, Ok(42));
        assert_eq!(attempts, 3);
    }

    #[test]
    fn test_retry_all_failures() {
        let result = retry(|| Err::<i32, String>("error".to_string()), 3);
        assert!(matches!(result, Err(RetryError::MaxRetriesExceeded(_))));
        
        if let Err(RetryError::MaxRetriesExceeded(errors)) = result {
            assert_eq!(errors.len(), 3);
        }
    }

    #[test]
    fn test_retry_zero_attempts() {
        let result = retry(|| Ok::<i32, String>(42), 0);
        assert!(matches!(result, Err(RetryError::MaxRetriesExceeded(_))));
    }

    #[test]
    fn test_retry_with_predicate_retryable() {
        let mut attempts = 0;
        let result = retry_with_predicate(
            || {
                attempts += 1;
                if attempts < 3 {
                    Err("transient")
                } else {
                    Ok(42)
                }
            },
            5,
            |e| *e == "transient",
        );
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_retry_with_predicate_permanent_error() {
        let result = retry_with_predicate(
            || Err::<i32, &str>("permanent"),
            5,
            |e| *e != "permanent",
        );
        assert_eq!(result, Err(RetryError::PermanentError("permanent")));
    }

    #[test]
    fn test_retry_with_predicate_max_retries() {
        let result = retry_with_predicate(
            || Err::<i32, &str>("transient"),
            3,
            |_| true,
        );
        assert!(matches!(result, Err(RetryError::MaxRetriesExceeded(_))));
    }
}
