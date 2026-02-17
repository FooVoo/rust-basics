//! Exercise 28: Enum with Associated Functions - Builder Pattern
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement builder pattern with enums
//! - Use associated functions effectively
//! - Create fluent interfaces with enums

#[derive(Debug, PartialEq, Clone)]
pub enum QueryBuilder {
    Empty,
    Select { fields: Vec<String> },
    SelectFrom { fields: Vec<String>, table: String },
    SelectFromWhere {
        fields: Vec<String>,
        table: String,
        condition: String,
    },
}

impl QueryBuilder {
    /// Creates a new empty query builder
    pub fn new() -> Self  {
        todo!("Create a new empty query builder")
    }

    /// Adds SELECT clause
    pub fn select(fields: Vec<String>) -> Self  {
        todo!("Add SELECT clause")
    }

    /// Adds FROM clause
    pub fn from(self, table: String) -> Result<Self, String>  {
        todo!("Add FROM clause")
    }

    /// Adds WHERE clause
    pub fn where_clause(self, condition: String) -> Result<Self, String>  {
        todo!("Add WHERE clause")
    }

    /// Builds the final SQL query string
    pub fn build(&self) -> Result<String, String>  {
        todo!("Build the final SQL query string")
    }
}

impl Default for QueryBuilder {
    fn default() -> Self  {
        todo!("Build the final SQL query string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_query() {
        let query = QueryBuilder::select(vec!["name".to_string(), "age".to_string()])
            .from("users".to_string())
            .unwrap();
        assert_eq!(query.build().unwrap(), "SELECT name, age FROM users");
    }

    #[test]
    fn test_query_with_where() {
        let query = QueryBuilder::select(vec!["name".to_string()])
            .from("users".to_string())
            .unwrap()
            .where_clause("age > 18".to_string())
            .unwrap();
        assert_eq!(
            query.build().unwrap(),
            "SELECT name FROM users WHERE age > 18"
        );
    }

    #[test]
    fn test_invalid_order() {
        let query = QueryBuilder::new();
        assert!(query.from("users".to_string()).is_err());
    }

    #[test]
    fn test_incomplete_query() {
        let query = QueryBuilder::select(vec!["name".to_string()]);
        assert!(query.build().is_err());
    }
}
