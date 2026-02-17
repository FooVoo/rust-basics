//! Exercise 22: Error Recovery - Implement fallback strategies
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement error recovery mechanisms
//! - Use fallback values and strategies
//! - Handle partial failures gracefully

#[derive(Debug, PartialEq, Clone)]
pub struct Data {
    pub value: i32,
    pub source: String,
}

#[derive(Debug, PartialEq)]
pub enum FetchError {
    NetworkError,
    ParseError,
    Timeout,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result  {
        todo!("Implement fmt")
    }
}

impl std::error::Error for FetchError {}

/// Try to fetch from primary source, fall back to secondary on error.
pub fn fetch_with_fallback<F1, F2>(
    primary: F1,
    fallback: F2,
) -> Result<Data, FetchError>
where
    F1: FnOnce() -> Result<Data, FetchError>,
    F2: FnOnce() -> Result<Data, FetchError>,
 {
    todo!("Try to fetch from primary source, fall back to secondary on error.")
}

/// Try multiple sources in order until one succeeds.
pub fn fetch_with_multiple_fallbacks(
    sources: Vec<fn() -> Result<Data, FetchError>>,
) -> Result<Data, FetchError>  {
    todo!("Try multiple sources in order until one succeeds.")
}

/// Fetch data or use default value on error.
pub fn fetch_or_default<F>(fetcher: F, default: Data) -> Data
where
    F: FnOnce() -> Result<Data, FetchError>,
 {
    todo!("Fetch data or use default value on error.")
}

/// Fetch multiple items, returning successes and collecting errors.
pub fn fetch_multiple<F>(
    fetchers: Vec<F>,
) -> (Vec<Data>, Vec<FetchError>)
where
    F: FnOnce() -> Result<Data, FetchError>,
 {
    todo!("Fetch multiple items, returning successes and collecting errors.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_with_fallback_primary_success() {
        let result = fetch_with_fallback(
            || Ok(Data { value: 1, source: "primary".to_string() }),
            || Ok(Data { value: 2, source: "fallback".to_string() }),
        );
        
        assert_eq!(result.unwrap().source, "primary");
    }

    #[test]
    fn test_fetch_with_fallback_primary_fails() {
        let result = fetch_with_fallback(
            || Err(FetchError::NetworkError),
            || Ok(Data { value: 2, source: "fallback".to_string() }),
        );
        
        assert_eq!(result.unwrap().source, "fallback");
    }

    #[test]
    fn test_fetch_with_fallback_both_fail() {
        let result = fetch_with_fallback(
            || Err(FetchError::NetworkError),
            || Err(FetchError::Timeout),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_with_multiple_fallbacks_first_succeeds() {
        let sources: Vec<fn() -> Result<Data, FetchError>> = vec![
            || Ok(Data { value: 1, source: "first".to_string() }),
            || Ok(Data { value: 2, source: "second".to_string() }),
        ];
        
        let result = fetch_with_multiple_fallbacks(sources);
        assert_eq!(result.unwrap().source, "first");
    }

    #[test]
    fn test_fetch_with_multiple_fallbacks_second_succeeds() {
        let sources: Vec<fn() -> Result<Data, FetchError>> = vec![
            || Err(FetchError::NetworkError),
            || Ok(Data { value: 2, source: "second".to_string() }),
            || Ok(Data { value: 3, source: "third".to_string() }),
        ];
        
        let result = fetch_with_multiple_fallbacks(sources);
        assert_eq!(result.unwrap().source, "second");
    }

    #[test]
    fn test_fetch_with_multiple_fallbacks_all_fail() {
        let sources: Vec<fn() -> Result<Data, FetchError>> = vec![
            || Err(FetchError::NetworkError),
            || Err(FetchError::Timeout),
        ];
        
        let result = fetch_with_multiple_fallbacks(sources);
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_or_default_success() {
        let default = Data { value: 0, source: "default".to_string() };
        let result = fetch_or_default(
            || Ok(Data { value: 1, source: "fetched".to_string() }),
            default,
        );
        
        assert_eq!(result.source, "fetched");
    }

    #[test]
    fn test_fetch_or_default_uses_default() {
        let default = Data { value: 0, source: "default".to_string() };
        let result = fetch_or_default(
            || Err(FetchError::NetworkError),
            default.clone(),
        );
        
        assert_eq!(result, default);
    }

    #[test]
    fn test_fetch_multiple_all_success() {
        let fetchers: Vec<Box<dyn FnOnce() -> Result<Data, FetchError>>> = vec![
            Box::new(|| Ok(Data { value: 1, source: "a".to_string() })),
            Box::new(|| Ok(Data { value: 2, source: "b".to_string() })),
        ];
        
        let (successes, errors) = fetch_multiple(fetchers);
        assert_eq!(successes.len(), 2);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_fetch_multiple_mixed() {
        let fetchers: Vec<Box<dyn FnOnce() -> Result<Data, FetchError>>> = vec![
            Box::new(|| Ok(Data { value: 1, source: "a".to_string() })),
            Box::new(|| Err(FetchError::NetworkError)),
            Box::new(|| Ok(Data { value: 2, source: "b".to_string() })),
        ];
        
        let (successes, errors) = fetch_multiple(fetchers);
        assert_eq!(successes.len(), 2);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_fetch_multiple_all_fail() {
        let fetchers: Vec<Box<dyn FnOnce() -> Result<Data, FetchError>>> = vec![
            Box::new(|| Err(FetchError::NetworkError)),
            Box::new(|| Err(FetchError::Timeout)),
        ];
        
        let (successes, errors) = fetch_multiple(fetchers);
        assert_eq!(successes.len(), 0);
        assert_eq!(errors.len(), 2);
    }
}
