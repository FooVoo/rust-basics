//! # Error Handling Exercises
//!
//! ## Learning Objectives
//! - Understand Result<T, E> and its use cases
//! - Master error propagation with the `?` operator
//! - Create custom error types
//! - Handle multiple error types effectively
//! - Use error handling combinators
//! - Implement proper error context and debugging
//!
//! ## Topics Covered
//! - Result and Option types
//! - Error propagation
//! - Custom error types
//! - Error conversion (From trait)
//! - Error combinators (map_err, and_then, or_else)
//! - anyhow and thiserror patterns
//!
//! ## Difficulty Distribution
//! - Easy: 5 exercises (01-05)
//! - Medium: 8 exercises (06-13)
//! - Hard: 5 exercises (14-18)
//! - Expert: 2 exercises (19-20)

// ============================================================================
// EASY EXERCISES (01-05): Basic Result and Error handling
// ============================================================================

/// Exercise 01: Basic Result - Parse a string to integer
/// Difficulty: Easy
///
/// Create a function that parses a string to an i32.
/// Return Ok(value) if successful, Err(message) otherwise.
pub fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

/// Exercise 02: Simple Validation - Check age validity
/// Difficulty: Easy
///
/// Create a function that validates an age (must be between 0 and 150).
/// Return Ok(age) if valid, Err with message otherwise.
pub fn validate_age(age: i32) -> Result<i32, String> {
    if age < 0 {
        Err("Age cannot be negative".to_string())
    } else if age > 150 {
        Err("Age cannot be greater than 150".to_string())
    } else {
        Ok(age)
    }
}

/// Exercise 03: Option to Result - Convert Option to Result
/// Difficulty: Easy
///
/// Create a function that converts an Option<T> to Result<T, String>.
pub fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
    opt.ok_or_else(|| error_msg.to_string())
}

/// Exercise 04: Safe Division - Divide two numbers safely
/// Difficulty: Easy
///
/// Create a function that safely divides two numbers.
/// Return Err if dividing by zero.
pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Exercise 05: Array Access - Safe array indexing
/// Difficulty: Easy
///
/// Create a function that safely accesses an array element.
pub fn safe_array_access<T: Clone>(arr: &[T], index: usize) -> Result<T, String> {
    arr.get(index)
        .cloned()
        .ok_or_else(|| format!("Index {} out of bounds", index))
}

// ============================================================================
// MEDIUM EXERCISES (06-13): Error propagation and conversion
// ============================================================================

/// Exercise 06: Error Propagation - Chain multiple operations
/// Difficulty: Medium
///
/// Create a function that parses and validates an age from a string.
pub fn parse_and_validate_age(s: &str) -> Result<i32, String> {
    let age = parse_number(s)?;
    validate_age(age)
}

/// Exercise 07: Multiple Validations - Validate username
/// Difficulty: Medium
///
/// Create a function that validates a username:
/// - Length must be between 3 and 20 characters
/// - Must contain only alphanumeric characters
pub fn validate_username(username: &str) -> Result<String, String> {
    if username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    if username.len() > 20 {
        return Err("Username must be at most 20 characters".to_string());
    }
    if !username.chars().all(|c| c.is_alphanumeric()) {
        return Err("Username must contain only alphanumeric characters".to_string());
    }
    Ok(username.to_string())
}

/// Exercise 08: Custom Error Type - Create a ParseError enum
/// Difficulty: Medium
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    EmptyInput,
    InvalidFormat,
    OutOfRange,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Input is empty"),
            ParseError::InvalidFormat => write!(f, "Invalid format"),
            ParseError::OutOfRange => write!(f, "Value out of range"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_percentage(s: &str) -> Result<u8, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    
    let num = s.parse::<u8>().map_err(|_| ParseError::InvalidFormat)?;
    
    if num > 100 {
        return Err(ParseError::OutOfRange);
    }
    
    Ok(num)
}

/// Exercise 09: Error Context - Add context to errors
/// Difficulty: Medium
///
/// Create a function that reads a "file" (simulated) and adds context.
pub fn read_config(filename: &str) -> Result<String, String> {
    if filename.is_empty() {
        return Err("Filename cannot be empty".to_string());
    }
    
    if !filename.ends_with(".conf") {
        return Err(format!("Invalid config file: {}", filename));
    }
    
    // Simulate reading
    Ok("config_data".to_string())
}

/// Exercise 10: Combining Results - Validate and parse multiple fields
/// Difficulty: Medium
#[derive(Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub age: i32,
}

pub fn create_user(username: &str, age_str: &str) -> Result<User, String> {
    let username = validate_username(username)?;
    let age = parse_and_validate_age(age_str)?;
    
    Ok(User { username, age })
}

/// Exercise 11: Collecting Results - Parse multiple numbers
/// Difficulty: Medium
///
/// Parse a vector of strings into a vector of numbers.
/// Return an error if any parsing fails.
pub fn parse_numbers(strings: &[&str]) -> Result<Vec<i32>, String> {
    strings.iter().map(|s| parse_number(s)).collect()
}

/// Exercise 12: Fallback Values - Provide default on error
/// Difficulty: Medium
///
/// Parse a number with a fallback default value.
pub fn parse_with_default(s: &str, default: i32) -> i32 {
    parse_number(s).unwrap_or(default)
}

/// Exercise 13: Early Return - Multiple validation checks
/// Difficulty: Medium
///
/// Validate email format (simplified version).
pub fn validate_email(email: &str) -> Result<String, String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    
    if !email.contains('@') {
        return Err("Email must contain @".to_string());
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err("Email must contain exactly one @".to_string());
    }
    
    if parts[0].is_empty() || parts[1].is_empty() {
        return Err("Email parts cannot be empty".to_string());
    }
    
    Ok(email.to_string())
}

// ============================================================================
// HARD EXERCISES (14-18): Complex error handling patterns
// ============================================================================

/// Exercise 14: Custom Error with From - Multiple error types
/// Difficulty: Hard
#[derive(Debug)]
pub enum AppError {
    Parse(std::num::ParseIntError),
    Validation(String),
    NotFound(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

pub fn process_user_age(age_str: &str) -> Result<i32, AppError> {
    let age: i32 = age_str.parse()?;
    
    if age < 0 || age > 150 {
        return Err(AppError::Validation("Age out of range".to_string()));
    }
    
    Ok(age)
}

/// Exercise 15: Nested Result Handling - Deep error propagation
/// Difficulty: Hard
pub fn parse_and_calculate(a: &str, b: &str, operation: &str) -> Result<f64, String> {
    let num_a = a.parse::<f64>().map_err(|_| format!("Invalid first number: {}", a))?;
    let num_b = b.parse::<f64>().map_err(|_| format!("Invalid second number: {}", b))?;
    
    match operation {
        "add" => Ok(num_a + num_b),
        "sub" => Ok(num_a - num_b),
        "mul" => Ok(num_a * num_b),
        "div" => safe_divide(num_a, num_b),
        _ => Err(format!("Unknown operation: {}", operation)),
    }
}

/// Exercise 16: Error Recovery - Try multiple strategies
/// Difficulty: Hard
pub fn parse_flexible(s: &str) -> Result<i32, String> {
    // Try direct parsing
    if let Ok(num) = s.parse::<i32>() {
        return Ok(num);
    }
    
    // Try removing whitespace
    let trimmed = s.trim();
    if let Ok(num) = trimmed.parse::<i32>() {
        return Ok(num);
    }
    
    // Try removing common prefixes
    let without_prefix = trimmed.trim_start_matches('+');
    if let Ok(num) = without_prefix.parse::<i32>() {
        return Ok(num);
    }
    
    Err(format!("Could not parse '{}' as number", s))
}

/// Exercise 17: Transaction-like Error Handling - All or nothing
/// Difficulty: Hard
#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub items: Vec<i32>,
}

pub fn process_transaction(inputs: &[&str]) -> Result<Transaction, String> {
    let items: Result<Vec<i32>, String> = inputs
        .iter()
        .map(|s| parse_number(s))
        .collect();
    
    let items = items?;
    
    // Validate all items are positive
    if items.iter().any(|&x| x <= 0) {
        return Err("All transaction items must be positive".to_string());
    }
    
    Ok(Transaction { items })
}

/// Exercise 18: Detailed Error Context - Build error chain
/// Difficulty: Hard
pub fn load_and_parse_config(filename: &str) -> Result<i32, String> {
    let content = read_config(filename)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    
    parse_number(&content)
        .map_err(|e| format!("Failed to parse config value '{}': {}", content, e))
}

// ============================================================================
// EXPERT EXERCISES (19-20): Advanced error handling
// ============================================================================

/// Exercise 19: Generic Error Handler - Work with any error type
/// Difficulty: Expert
pub fn handle_with_recovery<T, E, F>(
    result: Result<T, E>,
    recovery: F,
) -> Result<T, E>
where
    F: FnOnce(E) -> Result<T, E>,
{
    match result {
        Ok(val) => Ok(val),
        Err(e) => recovery(e),
    }
}

/// Exercise 20: Error Accumulation - Collect all errors
/// Difficulty: Expert
pub fn validate_all(inputs: &[&str]) -> Result<Vec<i32>, Vec<String>> {
    let mut successes = Vec::new();
    let mut errors = Vec::new();
    
    for input in inputs {
        match parse_number(input) {
            Ok(num) => successes.push(num),
            Err(e) => errors.push(format!("Failed to parse '{}': {}", input, e)),
        }
    }
    
    if errors.is_empty() {
        Ok(successes)
    } else {
        Err(errors)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Easy Tests
    #[test]
    fn test_parse_number_success() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("-10"), Ok(-10));
    }

    #[test]
    fn test_parse_number_failure() {
        assert!(parse_number("abc").is_err());
        assert!(parse_number("").is_err());
    }

    #[test]
    fn test_validate_age_valid() {
        assert_eq!(validate_age(25), Ok(25));
        assert_eq!(validate_age(0), Ok(0));
        assert_eq!(validate_age(150), Ok(150));
    }

    #[test]
    fn test_validate_age_invalid() {
        assert!(validate_age(-1).is_err());
        assert!(validate_age(151).is_err());
    }

    #[test]
    fn test_option_to_result() {
        assert_eq!(option_to_result(Some(42), "error"), Ok(42));
        assert_eq!(option_to_result::<i32>(None, "not found"), Err("not found".to_string()));
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_safe_array_access() {
        let arr = vec![1, 2, 3];
        assert_eq!(safe_array_access(&arr, 1), Ok(2));
        assert!(safe_array_access(&arr, 5).is_err());
    }

    // Medium Tests
    #[test]
    fn test_parse_and_validate_age() {
        assert_eq!(parse_and_validate_age("25"), Ok(25));
        assert!(parse_and_validate_age("abc").is_err());
        assert!(parse_and_validate_age("200").is_err());
    }

    #[test]
    fn test_validate_username() {
        assert_eq!(validate_username("user123"), Ok("user123".to_string()));
        assert!(validate_username("ab").is_err());
        assert!(validate_username("user@123").is_err());
    }

    #[test]
    fn test_parse_percentage() {
        assert_eq!(parse_percentage("50"), Ok(50));
        assert_eq!(parse_percentage(""), Err(ParseError::EmptyInput));
        assert_eq!(parse_percentage("abc"), Err(ParseError::InvalidFormat));
        assert_eq!(parse_percentage("150"), Err(ParseError::OutOfRange));
    }

    #[test]
    fn test_read_config() {
        assert!(read_config("app.conf").is_ok());
        assert!(read_config("").is_err());
        assert!(read_config("app.txt").is_err());
    }

    #[test]
    fn test_create_user() {
        let user = create_user("alice", "30").unwrap();
        assert_eq!(user.username, "alice");
        assert_eq!(user.age, 30);
        
        assert!(create_user("ab", "30").is_err());
        assert!(create_user("alice", "abc").is_err());
    }

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
        assert!(parse_numbers(&["1", "abc", "3"]).is_err());
    }

    #[test]
    fn test_parse_with_default() {
        assert_eq!(parse_with_default("42", 0), 42);
        assert_eq!(parse_with_default("abc", 10), 10);
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("").is_err());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    // Hard Tests
    #[test]
    fn test_process_user_age() {
        assert_eq!(process_user_age("25").unwrap(), 25);
        assert!(matches!(process_user_age("abc"), Err(AppError::Parse(_))));
        assert!(matches!(process_user_age("200"), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_parse_and_calculate() {
        assert_eq!(parse_and_calculate("10", "5", "add"), Ok(15.0));
        assert_eq!(parse_and_calculate("10", "5", "sub"), Ok(5.0));
        assert_eq!(parse_and_calculate("10", "5", "mul"), Ok(50.0));
        assert_eq!(parse_and_calculate("10", "5", "div"), Ok(2.0));
        assert!(parse_and_calculate("10", "0", "div").is_err());
        assert!(parse_and_calculate("10", "5", "unknown").is_err());
    }

    #[test]
    fn test_parse_flexible() {
        assert_eq!(parse_flexible("42"), Ok(42));
        assert_eq!(parse_flexible("  42  "), Ok(42));
        assert_eq!(parse_flexible("+42"), Ok(42));
        assert!(parse_flexible("abc").is_err());
    }

    #[test]
    fn test_process_transaction() {
        let result = process_transaction(&["1", "2", "3"]).unwrap();
        assert_eq!(result.items, vec![1, 2, 3]);
        
        assert!(process_transaction(&["1", "abc", "3"]).is_err());
        assert!(process_transaction(&["1", "-5", "3"]).is_err());
    }

    #[test]
    fn test_load_and_parse_config() {
        assert!(load_and_parse_config("app.conf").is_ok());
        assert!(load_and_parse_config("app.txt").is_err());
    }

    // Expert Tests
    #[test]
    fn test_handle_with_recovery() {
        let result: Result<i32, String> = Err("error".to_string());
        let recovered = handle_with_recovery(result, |_| Ok(42));
        assert_eq!(recovered, Ok(42));
        
        let ok_result: Result<i32, String> = Ok(10);
        let not_recovered = handle_with_recovery(ok_result, |_| Ok(42));
        assert_eq!(not_recovered, Ok(10));
    }

    #[test]
    fn test_validate_all() {
        assert_eq!(validate_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
        
        let result = validate_all(&["1", "abc", "3", "def"]);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 2);
        }
    }
}
