//! Exercise 23: Context Propagation - Add context at each layer
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Add contextual information at each error layer
//! - Build informative error messages
//! - Track error propagation through stack

use std::fmt;

#[derive(Debug)]
pub struct ContextError {
    message: String,
    context: Vec<String>,
}

impl ContextError {
    pub fn new(message: impl Into<String>) -> Self {
        ContextError {
            message: message.into(),
            context: Vec::new(),
        }
    }
    
    pub fn context(mut self, ctx: impl Into<String>) -> Self {
        self.context.push(ctx.into());
        self
    }
    
    pub fn with_context<F>(result: Result<(), String>, ctx: F) -> Result<(), Self>
    where
        F: FnOnce() -> String,
    {
        result.map_err(|msg| ContextError::new(msg).context(ctx()))
    }
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;
        for (i, ctx) in self.context.iter().rev().enumerate() {
            if i == 0 {
                write!(f, "\n  Context: {}", ctx)?;
            } else {
                write!(f, "\n  ... in: {}", ctx)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for ContextError {}

/// Layer 1: Low-level operation
fn read_file(path: &str) -> Result<String, ContextError> {
    if path.is_empty() {
        Err(ContextError::new("Path is empty")
            .context("read_file operation"))
    } else if path.starts_with('/') {
        Ok("file contents".to_string())
    } else {
        Err(ContextError::new("Invalid path format")
            .context("read_file operation"))
    }
}

/// Layer 2: Mid-level processing
pub fn parse_config(path: &str) -> Result<String, ContextError> {
    let contents = read_file(path)
        .map_err(|e| e.context("parse_config function"))?;
    
    if contents.is_empty() {
        Err(ContextError::new("Empty config file")
            .context("parse_config function"))
    } else {
        Ok(contents)
    }
}

/// Layer 3: High-level application
pub fn initialize_app(config_path: &str) -> Result<String, ContextError> {
    let config = parse_config(config_path)
        .map_err(|e| e.context("initialize_app"))?;
    
    Ok(config)
}

/// Add context to multiple operations
pub fn process_batch(paths: &[&str]) -> Result<Vec<String>, Vec<ContextError>> {
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for (i, path) in paths.iter().enumerate() {
        match initialize_app(path) {
            Ok(config) => results.push(config),
            Err(e) => errors.push(e.context(format!("Processing item {}", i))),
        }
    }
    
    if errors.is_empty() {
        Ok(results)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_success() {
        let result = read_file("/valid/path");
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_file_empty_path() {
        let result = read_file("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Path is empty"));
        assert!(err.to_string().contains("read_file"));
    }

    #[test]
    fn test_parse_config_success() {
        let result = parse_config("/valid/path");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_config_invalid_path() {
        let result = parse_config("invalid");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_str = err.to_string();
        assert!(err_str.contains("Invalid path format"));
        assert!(err_str.contains("parse_config"));
    }

    #[test]
    fn test_initialize_app_success() {
        let result = initialize_app("/valid/path");
        assert!(result.is_ok());
    }

    #[test]
    fn test_initialize_app_error_context() {
        let result = initialize_app("invalid");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_str = err.to_string();
        assert!(err_str.contains("initialize_app"));
        assert!(err_str.contains("parse_config"));
        assert!(err_str.contains("read_file"));
    }

    #[test]
    fn test_context_chain_depth() {
        let result = initialize_app("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.context.len(), 3);
    }

    #[test]
    fn test_process_batch_all_success() {
        let paths = &["/path1", "/path2", "/path3"];
        let result = process_batch(paths);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_process_batch_some_errors() {
        let paths = &["/path1", "invalid", "/path3"];
        let result = process_batch(paths);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].to_string().contains("Processing item 1"));
    }

    #[test]
    fn test_process_batch_all_errors() {
        let paths = &["", "invalid"];
        let result = process_batch(paths);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_error_display_format() {
        let err = ContextError::new("Base error")
            .context("First layer")
            .context("Second layer");
        
        let display = err.to_string();
        assert!(display.contains("Base error"));
        assert!(display.contains("Context:"));
    }
}
