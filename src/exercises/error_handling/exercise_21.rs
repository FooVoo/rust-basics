//! Exercise 21: Error Chain - Track error chains and causes
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Build error chains with source tracking
//! - Implement Error trait with source method
//! - Trace error origins through layers

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Implement fmt")
    }
}

impl Error for DatabaseError {}

#[derive(Debug)]
pub enum ServiceError {
    Database(DatabaseError),
    Validation(String),
    NotFound(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Implement fmt")
    }
}

impl Error for ServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("Implement source")
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(err: DatabaseError) -> Self {
        todo!("Implement from")
    }
}

/// Simulate a database operation that can fail.
pub fn database_query(success: bool) -> Result<String, DatabaseError> {
    todo!("Implement database_query")
}

/// Service layer that wraps database operations.
pub fn fetch_user(id: u32, db_success: bool) -> Result<String, ServiceError> {
    todo!("Implement fetch_user")
}

/// Count the depth of error chain.
pub fn error_chain_depth(err: &dyn Error) -> usize {
    todo!("Implement error_chain_depth")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_query_success() {
        let result = database_query(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "data");
    }

    #[test]
    fn test_database_query_failure() {
        let result = database_query(false);
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_user_success() {
        let result = fetch_user(1, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fetch_user_validation_error() {
        let result = fetch_user(0, true);
        assert!(result.is_err());
        match result.unwrap_err() {
            ServiceError::Validation(_) => (),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_fetch_user_database_error() {
        let result = fetch_user(1, false);
        assert!(result.is_err());
        match result.unwrap_err() {
            ServiceError::Database(_) => (),
            _ => panic!("Expected database error"),
        }
    }

    #[test]
    fn test_error_chain_depth_single() {
        let err = DatabaseError::ConnectionError("test".to_string());
        assert_eq!(error_chain_depth(&err), 1);
    }

    #[test]
    fn test_error_chain_depth_nested() {
        let db_err = DatabaseError::ConnectionError("test".to_string());
        let service_err = ServiceError::Database(db_err);
        assert_eq!(error_chain_depth(&service_err), 2);
    }

    #[test]
    fn test_error_source() {
        let db_err = DatabaseError::QueryError("test".to_string());
        let service_err = ServiceError::Database(db_err);
        
        assert!(service_err.source().is_some());
    }

    #[test]
    fn test_error_display() {
        let err = ServiceError::Validation("test".to_string());
        assert!(err.to_string().contains("Validation error"));
        
        let db_err = DatabaseError::ConnectionError("failed".to_string());
        let service_err = ServiceError::Database(db_err);
        assert!(service_err.to_string().contains("Service database error"));
    }
}
