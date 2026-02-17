//! Exercise 27: Error Logging and Debugging - Track errors for debugging
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Add logging to error paths
//! - Track error metadata
//! - Build debuggable error types

use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorLog {
    pub message: String,
    pub timestamp: SystemTime,
    pub severity: ErrorSeverity,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
}

impl ErrorLog {
    pub fn new(
        message: impl Into<String>,
        severity: ErrorSeverity,
        location: impl Into<String>,
    ) -> Self  {
        todo!("Implement new")
    }
}

impl fmt::Display for ErrorLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        todo!("Implement fmt")
    }
}

/// A Result type that includes error logging.
pub type LoggedResult<T> = Result<T, ErrorLog>;

/// Logger that tracks all errors.
#[derive(Debug, Default)]
pub struct ErrorLogger {
    logs: Vec<ErrorLog>,
}

impl ErrorLogger {
    pub fn new() -> Self  {
        todo!("Logger that tracks all errors.")
    }
    
    pub fn log(&mut self, log: ErrorLog)  {
        todo!("Implement log")
    }
    
    pub fn logs(&self) -> &[ErrorLog]  {
        todo!("Implement logs")
    }
    
    pub fn count_by_severity(&self, severity: ErrorSeverity) -> usize  {
        todo!("Implement count_by_severity")
    }
    
    pub fn has_critical_errors(&self) -> bool  {
        todo!("Implement has_critical_errors")
    }
    
    pub fn clear(&mut self)  {
        todo!("Implement clear")
    }
}

/// Perform an operation with error logging.
pub fn with_logging<F, T>(
    logger: &mut ErrorLogger,
    location: &str,
    operation: F,
) -> LoggedResult<T>
where
    F: FnOnce() -> LoggedResult<T>,
 {
    todo!("Perform an operation with error logging.")
}

/// Parse a number with logging.
pub fn parse_logged(s: &str, logger: &mut ErrorLogger) -> LoggedResult<i32>  {
    todo!("Parse a number with logging.")
}

/// Divide numbers with logging.
pub fn divide_logged(a: i32, b: i32, logger: &mut ErrorLogger) -> LoggedResult<i32>  {
    todo!("Divide numbers with logging.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_log_creation() {
        let log = ErrorLog::new("Test error", ErrorSeverity::Error, "test_function");
        assert_eq!(log.message, "Test error");
        assert_eq!(log.severity, ErrorSeverity::Error);
        assert_eq!(log.location, "test_function");
    }

    #[test]
    fn test_error_logger_empty() {
        let logger = ErrorLogger::new();
        assert_eq!(logger.logs().len(), 0);
        assert!(!logger.has_critical_errors());
    }

    #[test]
    fn test_error_logger_log() {
        let mut logger = ErrorLogger::new();
        logger.log(ErrorLog::new("Error 1", ErrorSeverity::Error, "loc1"));
        logger.log(ErrorLog::new("Error 2", ErrorSeverity::Warning, "loc2"));
        
        assert_eq!(logger.logs().len(), 2);
    }

    #[test]
    fn test_error_logger_count_by_severity() {
        let mut logger = ErrorLogger::new();
        logger.log(ErrorLog::new("E1", ErrorSeverity::Error, "loc"));
        logger.log(ErrorLog::new("W1", ErrorSeverity::Warning, "loc"));
        logger.log(ErrorLog::new("E2", ErrorSeverity::Error, "loc"));
        logger.log(ErrorLog::new("C1", ErrorSeverity::Critical, "loc"));
        
        assert_eq!(logger.count_by_severity(ErrorSeverity::Error), 2);
        assert_eq!(logger.count_by_severity(ErrorSeverity::Warning), 1);
        assert_eq!(logger.count_by_severity(ErrorSeverity::Critical), 1);
    }

    #[test]
    fn test_error_logger_has_critical() {
        let mut logger = ErrorLogger::new();
        logger.log(ErrorLog::new("E1", ErrorSeverity::Error, "loc"));
        assert!(!logger.has_critical_errors());
        
        logger.log(ErrorLog::new("C1", ErrorSeverity::Critical, "loc"));
        assert!(logger.has_critical_errors());
    }

    #[test]
    fn test_error_logger_clear() {
        let mut logger = ErrorLogger::new();
        logger.log(ErrorLog::new("E1", ErrorSeverity::Error, "loc"));
        assert_eq!(logger.logs().len(), 1);
        
        logger.clear();
        assert_eq!(logger.logs().len(), 0);
    }

    #[test]
    fn test_parse_logged_success() {
        let mut logger = ErrorLogger::new();
        let result = parse_logged("42", &mut logger);
        
        assert_eq!(result, Ok(42));
        assert_eq!(logger.logs().len(), 0);
    }

    #[test]
    fn test_parse_logged_error() {
        let mut logger = ErrorLogger::new();
        let result = parse_logged("abc", &mut logger);
        
        assert!(result.is_err());
        assert_eq!(logger.logs().len(), 1);
        assert_eq!(logger.logs()[0].severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_divide_logged_success() {
        let mut logger = ErrorLogger::new();
        let result = divide_logged(10, 2, &mut logger);
        
        assert_eq!(result, Ok(5));
        assert_eq!(logger.logs().len(), 0);
    }

    #[test]
    fn test_divide_logged_division_by_zero() {
        let mut logger = ErrorLogger::new();
        let result = divide_logged(10, 0, &mut logger);
        
        assert!(result.is_err());
        assert_eq!(logger.logs().len(), 1);
        assert_eq!(logger.logs()[0].severity, ErrorSeverity::Critical);
    }

    #[test]
    fn test_multiple_operations_logging() {
        let mut logger = ErrorLogger::new();
        
        let _ = parse_logged("42", &mut logger);
        let _ = parse_logged("abc", &mut logger);
        let _ = divide_logged(10, 0, &mut logger);
        let _ = divide_logged(10, 2, &mut logger);
        
        assert_eq!(logger.logs().len(), 2);
        assert_eq!(logger.count_by_severity(ErrorSeverity::Error), 1);
        assert_eq!(logger.count_by_severity(ErrorSeverity::Critical), 1);
    }
}
