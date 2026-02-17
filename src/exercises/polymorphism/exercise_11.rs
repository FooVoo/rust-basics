//! Exercise 11: Default Trait Methods - Define default implementations in traits
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Define default method implementations
//! - Override default methods
//! - Use default methods with custom implementations

pub trait Logger {
    fn log(&self, message: &str) -> String;
    
    // Default implementation
    fn log_info(&self, message: &str) -> String {
        todo!("Implement log_info")
    }
    
    // Default implementation
    fn log_error(&self, message: &str) -> String {
        todo!("Implement log_error")
    }
    
    // Default implementation that calls other methods
    fn log_debug(&self, message: &str) -> String {
        todo!("Implement log_debug")
    }
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) -> String {
        todo!("Implement log")
    }
}

pub struct FileLogger {
    pub filename: String,
}

impl Logger for FileLogger {
    fn log(&self, message: &str) -> String {
        todo!("Implement log")
    }
    
    // Override default method
    fn log_error(&self, message: &str) -> String {
        todo!("Implement log_error")
    }
}

pub struct SilentLogger;

impl Logger for SilentLogger {
    fn log(&self, _message: &str) -> String {
        todo!("Implement log")
    }
    
    // Override all methods to be silent
    fn log_info(&self, _message: &str) -> String {
        todo!("Implement log_info")
    }
    
    fn log_error(&self, _message: &str) -> String {
        todo!("Implement log_error")
    }
    
    fn log_debug(&self, _message: &str) -> String {
        todo!("Implement log_debug")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_logger_basic() {
        let logger = ConsoleLogger;
        assert_eq!(logger.log("test"), "[Console] test");
    }

    #[test]
    fn test_console_logger_info() {
        let logger = ConsoleLogger;
        let result = logger.log_info("information");
        assert!(result.contains("[Console]"));
        assert!(result.contains("INFO:"));
        assert!(result.contains("information"));
    }

    #[test]
    fn test_console_logger_error() {
        let logger = ConsoleLogger;
        let result = logger.log_error("problem");
        assert!(result.contains("ERROR:"));
    }

    #[test]
    fn test_file_logger_basic() {
        let logger = FileLogger { filename: "app.log".to_string() };
        assert!(logger.log("test").contains("app.log"));
    }

    #[test]
    fn test_file_logger_error_override() {
        let logger = FileLogger { filename: "errors.log".to_string() };
        let result = logger.log_error("critical");
        assert!(result.contains("!!!"));
        assert!(result.contains("ERROR"));
        assert!(result.contains("critical"));
    }

    #[test]
    fn test_file_logger_info_default() {
        let logger = FileLogger { filename: "info.log".to_string() };
        let result = logger.log_info("data");
        assert!(result.contains("INFO:"));
    }

    #[test]
    fn test_silent_logger() {
        let logger = SilentLogger;
        assert_eq!(logger.log("anything"), "");
        assert_eq!(logger.log_info("info"), "");
        assert_eq!(logger.log_error("error"), "");
        assert_eq!(logger.log_debug("debug"), "");
    }

    #[test]
    fn test_different_loggers() {
        let console = ConsoleLogger;
        let file = FileLogger { filename: "test.log".to_string() };
        
        assert!(console.log_info("test").contains("[Console]"));
        assert!(file.log_info("test").contains("test.log"));
    }
}
