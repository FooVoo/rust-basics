//! Exercise 20: Enum Serialization - Converting to/from Strings
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Convert enums to string representations
//! - Parse strings to create enum values
//! - Handle serialization errors

#[derive(Debug, PartialEq, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    /// Converts log level to string
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        }
    }

    /// Parses a string into a LogLevel
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO" => Ok(LogLevel::Info),
            "WARNING" => Ok(LogLevel::Warning),
            "ERROR" => Ok(LogLevel::Error),
            _ => Err(format!("Unknown log level: {}", s)),
        }
    }

    /// Returns numeric value for the log level
    pub fn to_number(&self) -> u8 {
        match self {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
        }
    }
}

/// Filters log messages by minimum level
pub fn filter_logs(logs: Vec<(LogLevel, String)>, min_level: LogLevel) -> Vec<String> {
    logs.into_iter()
        .filter(|(level, _)| level.to_number() >= min_level.to_number())
        .map(|(_, msg)| msg)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Warning.as_str(), "WARNING");
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(LogLevel::from_str("DEBUG"), Ok(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("info"), Ok(LogLevel::Info));
        assert_eq!(LogLevel::from_str("Warning"), Ok(LogLevel::Warning));
        assert!(LogLevel::from_str("UNKNOWN").is_err());
    }

    #[test]
    fn test_to_number() {
        assert_eq!(LogLevel::Debug.to_number(), 0);
        assert_eq!(LogLevel::Info.to_number(), 1);
        assert_eq!(LogLevel::Warning.to_number(), 2);
        assert_eq!(LogLevel::Error.to_number(), 3);
    }

    #[test]
    fn test_filter_logs() {
        let logs = vec![
            (LogLevel::Debug, "debug msg".to_string()),
            (LogLevel::Info, "info msg".to_string()),
            (LogLevel::Warning, "warning msg".to_string()),
            (LogLevel::Error, "error msg".to_string()),
        ];
        let filtered = filter_logs(logs, LogLevel::Warning);
        assert_eq!(filtered, vec!["warning msg", "error msg"]);
    }
}
