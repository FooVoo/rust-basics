//! # Combinator Exercises
//!
//! ## Learning Objectives
//! - Master Option and Result combinators
//! - Use functional programming patterns effectively
//! - Chain operations with map, and_then, or_else
//! - Handle errors elegantly with combinators
//! - Create custom combinators for complex workflows
//! - Understand when to use each combinator pattern
//!
//! ## Topics Covered
//! - Option combinators (map, and_then, or, or_else, filter)
//! - Result combinators (map, map_err, and_then, or, or_else)
//! - Chaining multiple combinators
//! - Error handling without explicit match
//! - Converting between Option and Result
//! - Custom combinator patterns
//! - Combinator composition and reusability
//! - Performance considerations
//!
//! ## Combinator Reference
//!
//! ### Option Combinators
//! - `map`: Transform the inner value if Some
//! - `and_then` (flatMap): Chain operations that return Option
//! - `or`: Provide alternative Option if None
//! - `or_else`: Compute alternative Option if None
//! - `filter`: Keep Some only if predicate passes
//! - `unwrap_or`: Provide default value if None
//! - `unwrap_or_else`: Compute default value if None
//!
//! ### Result Combinators
//! - `map`: Transform Ok value, keep Err unchanged
//! - `map_err`: Transform Err value, keep Ok unchanged
//! - `and_then`: Chain operations that return Result
//! - `or`: Provide alternative Result if Err
//! - `or_else`: Compute alternative Result if Err
//! - `unwrap_or`: Provide default value if Err
//! - `unwrap_or_else`: Compute default value if Err
//!
//! ## Difficulty Distribution
//! - Easy: 5 exercises (01-05)
//! - Medium: 8 exercises (06-13)
//! - Hard: 5 exercises (14-18)
//! - Expert: 2 exercises (19-20)

// ============================================================================
// EASY EXERCISES (01-05): Basic Option and Result combinators
// ============================================================================

/// Exercise 01: Basic Option Map - Transform Some values
/// Difficulty: Easy
///
/// Use `map` to double the number if it exists.
pub fn double_option(num: Option<i32>) -> Option<i32> {
    num.map(|n| n * 2)
}

/// Exercise 02: Option with Default - Use unwrap_or
/// Difficulty: Easy
///
/// Return the number if Some, otherwise return 0.
pub fn option_or_zero(num: Option<i32>) -> i32 {
    num.unwrap_or(0)
}

/// Exercise 03: Basic Result Map - Transform Ok values
/// Difficulty: Easy
///
/// Use `map` to add 10 to the Ok value.
pub fn add_ten_to_result(result: Result<i32, String>) -> Result<i32, String> {
    result.map(|n| n + 10)
}

/// Exercise 04: Filter Option - Keep Some only if condition met
/// Difficulty: Easy
///
/// Use `filter` to keep only positive numbers.
pub fn filter_positive(num: Option<i32>) -> Option<i32> {
    num.filter(|&n| n > 0)
}

/// Exercise 05: Result with Default - Use unwrap_or
/// Difficulty: Easy
///
/// Return the Ok value or -1 if Err.
pub fn result_or_minus_one(result: Result<i32, String>) -> i32 {
    result.unwrap_or(-1)
}

// ============================================================================
// MEDIUM EXERCISES (06-13): Chaining combinators and error handling
// ============================================================================

/// Exercise 06: Chain Option Combinators - Use and_then
/// Difficulty: Medium
///
/// Parse a string to a number, then divide by 2 if even.
pub fn parse_and_halve_if_even(s: &str) -> Option<i32> {
    s.parse::<i32>()
        .ok()
        .filter(|&n| n % 2 == 0)
        .map(|n| n / 2)
}

/// Exercise 07: Result Chain with and_then - Multiple operations
/// Difficulty: Medium
///
/// Parse a string to a number, then divide by a divisor.
/// Return Err if parsing fails or division by zero.
pub fn parse_and_divide(s: &str, divisor: i32) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))
        .and_then(|n| {
            if divisor == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(n / divisor)
            }
        })
}

/// Exercise 08: Option or_else - Compute fallback
/// Difficulty: Medium
///
/// Try to get the first option, if None try the second.
pub fn first_or_second(first: Option<i32>, second: fn() -> Option<i32>) -> Option<i32> {
    first.or_else(second)
}

/// Exercise 09: Map Error Types - Use map_err
/// Difficulty: Medium
///
/// Parse a string and convert ParseIntError to a custom message.
pub fn parse_with_custom_error(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Failed to parse '{}'", s))
}

/// Exercise 10: Complex Option Chain
/// Difficulty: Medium
///
/// Given an optional string, parse it to a number, square it if positive.
pub fn parse_square_positive(s: Option<&str>) -> Option<i32> {
    s.and_then(|string| string.parse::<i32>().ok())
        .filter(|&n| n > 0)
        .map(|n| n * n)
}

/// Exercise 11: Result or for Fallback
/// Difficulty: Medium
///
/// Try the first result, if it's Err use the second.
pub fn first_result_or_second(
    first: Result<i32, String>,
    second: Result<i32, String>,
) -> Result<i32, String> {
    first.or(second)
}

/// Exercise 12: Transpose Option and Result
/// Difficulty: Medium
///
/// Convert Option<Result<T, E>> to Result<Option<T>, E>
pub fn transpose_option_result(opt: Option<Result<i32, String>>) -> Result<Option<i32>, String> {
    opt.map_or(Ok(None), |res| res.map(Some))
}

/// Exercise 13: Combine Multiple Options
/// Difficulty: Medium
///
/// Add two optional numbers, return None if either is None.
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.and_then(|x| b.map(|y| x + y))
}

// ============================================================================
// HARD EXERCISES (14-18): Advanced patterns and custom combinators
// ============================================================================

/// Exercise 14: Custom Validation Chain
/// Difficulty: Hard
///
/// Validate a username: must be 3-20 chars, alphanumeric or underscore only.
/// Chain multiple validation steps using Result combinators.
pub fn validate_username(username: &str) -> Result<String, String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    Ok(username.to_string())
        .and_then(|u| {
            if u.len() >= 3 && u.len() <= 20 {
                Ok(u)
            } else {
                Err("Username must be 3-20 characters".to_string())
            }
        })
        .and_then(|u| {
            if u.chars().all(|c| c.is_alphanumeric() || c == '_') {
                Ok(u)
            } else {
                Err("Username must be alphanumeric".to_string())
            }
        })
}

/// Exercise 15: Safe Division with Combinators
/// Difficulty: Hard
///
/// Perform safe division: parse two strings, divide, handle all errors.
pub fn safe_divide(numerator: &str, denominator: &str) -> Result<f64, String> {
    numerator
        .parse::<f64>()
        .map_err(|_| format!("Invalid numerator: {}", numerator))
        .and_then(|num| {
            denominator
                .parse::<f64>()
                .map_err(|_| format!("Invalid denominator: {}", denominator))
                .and_then(|den| {
                    if den == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(num / den)
                    }
                })
        })
}

/// Exercise 16: Custom Combinator - Try Multiple Operations
/// Difficulty: Hard
///
/// Try parsing as different types until one succeeds.
pub fn parse_flexible(s: &str) -> Option<i32> {
    // Try parsing directly
    s.parse::<i32>()
        .ok()
        // Try removing whitespace
        .or_else(|| s.trim().parse::<i32>().ok())
        // Try parsing as float and converting
        .or_else(|| s.parse::<f64>().ok().map(|f| f as i32))
}

/// Exercise 17: Result Chain with Early Return Pattern
/// Difficulty: Hard
///
/// Process a config: parse port, validate range, create config.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub port: u16,
}

pub fn create_config(port_str: &str) -> Result<Config, String> {
    port_str
        .parse::<u16>()
        .map_err(|_| "Invalid port number".to_string())
        .and_then(|port| {
            if port < 1024 {
                Err("Port must be >= 1024".to_string())
            } else {
                Ok(Config { port })
            }
        })
}

/// Exercise 18: Nested Option/Result Handling
/// Difficulty: Hard
///
/// Parse a nested structure: Option<Result<Option<i32>, E>>
pub fn flatten_nested(
    nested: Option<Result<Option<i32>, String>>,
) -> Result<Option<i32>, String> {
    match nested {
        None => Ok(None),
        Some(Ok(inner)) => Ok(inner),
        Some(Err(e)) => Err(e),
    }
}

// ============================================================================
// EXPERT EXERCISES (19-20): Complex custom combinators
// ============================================================================

/// Exercise 19: Custom Combinator Trait - Tap for Side Effects
/// Difficulty: Expert
///
/// Implement a "tap" combinator that allows side effects in chains.
pub trait Tap: Sized {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self);
}

impl<T> Tap for Option<T> {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self),
    {
        f(&self);
        self
    }
}

impl<T, E> Tap for Result<T, E> {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self),
    {
        f(&self);
        self
    }
}

/// Helper function demonstrating tap usage
pub fn process_with_logging(value: Option<i32>) -> Option<i32> {
    value
        .tap(|v| println!("Processing: {:?}", v))
        .map(|n| n * 2)
        .tap(|v| println!("After doubling: {:?}", v))
        .filter(|&n| n > 10)
        .tap(|v| println!("After filtering: {:?}", v))
}

/// Exercise 20: Railway-Oriented Programming Pattern
/// Difficulty: Expert
///
/// Implement a complete validation pipeline using Result combinators.
/// This demonstrates "railway-oriented programming" - a functional approach
/// where success stays on one track and errors switch to another.
#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub age: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidationError {
    InvalidUsername(String),
    InvalidEmail(String),
    InvalidAge(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidUsername(msg) => write!(f, "Username error: {}", msg),
            ValidationError::InvalidEmail(msg) => write!(f, "Email error: {}", msg),
            ValidationError::InvalidAge(msg) => write!(f, "Age error: {}", msg),
        }
    }
}

/// Validate username
fn validate_username_field(username: &str) -> Result<String, ValidationError> {
    if username.is_empty() {
        Err(ValidationError::InvalidUsername(
            "Username cannot be empty".to_string(),
        ))
    } else if username.len() < 3 {
        Err(ValidationError::InvalidUsername(
            "Username too short".to_string(),
        ))
    } else if username.len() > 20 {
        Err(ValidationError::InvalidUsername(
            "Username too long".to_string(),
        ))
    } else if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Err(ValidationError::InvalidUsername(
            "Username must be alphanumeric".to_string(),
        ))
    } else {
        Ok(username.to_string())
    }
}

/// Validate email
fn validate_email_field(email: &str) -> Result<String, ValidationError> {
    if email.is_empty() {
        Err(ValidationError::InvalidEmail(
            "Email cannot be empty".to_string(),
        ))
    } else if !email.contains('@') {
        Err(ValidationError::InvalidEmail(
            "Email must contain @".to_string(),
        ))
    } else if !email.contains('.') {
        Err(ValidationError::InvalidEmail(
            "Email must contain domain".to_string(),
        ))
    } else {
        Ok(email.to_string())
    }
}

/// Validate age
fn validate_age_field(age: u8) -> Result<u8, ValidationError> {
    if age < 13 {
        Err(ValidationError::InvalidAge(
            "Must be at least 13".to_string(),
        ))
    } else if age > 120 {
        Err(ValidationError::InvalidAge("Invalid age".to_string()))
    } else {
        Ok(age)
    }
}

/// Create user with full validation pipeline
pub fn create_validated_user(
    username: &str,
    email: &str,
    age: u8,
) -> Result<User, ValidationError> {
    // Railway-oriented programming: chain validations
    validate_username_field(username).and_then(|valid_username| {
        validate_email_field(email).and_then(|valid_email| {
            validate_age_field(age).map(|valid_age| User {
                username: valid_username,
                email: valid_email,
                age: valid_age,
            })
        })
    })
}

/// Alternative approach: collect all errors
pub fn create_user_collect_errors(
    username: &str,
    email: &str,
    age: u8,
) -> Result<User, Vec<ValidationError>> {
    let username_result = validate_username_field(username);
    let email_result = validate_email_field(email);
    let age_result = validate_age_field(age);

    // Collect all errors
    let errors: Vec<ValidationError> = [
        username_result.as_ref().err(),
        email_result.as_ref().err(),
        age_result.as_ref().err(),
    ]
    .iter()
    .filter_map(|e| e.cloned())
    .collect();

    if errors.is_empty() {
        // All validations passed, extract the Ok values
        Ok(User {
            username: username_result.unwrap(),
            email: email_result.unwrap(),
            age: age_result.unwrap(),
        })
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
    fn test_double_option() {
        assert_eq!(double_option(Some(5)), Some(10));
        assert_eq!(double_option(Some(0)), Some(0));
        assert_eq!(double_option(None), None);
        assert_eq!(double_option(Some(-3)), Some(-6));
    }

    #[test]
    fn test_option_or_zero() {
        assert_eq!(option_or_zero(Some(42)), 42);
        assert_eq!(option_or_zero(None), 0);
        assert_eq!(option_or_zero(Some(-5)), -5);
        assert_eq!(option_or_zero(Some(0)), 0);
    }

    #[test]
    fn test_add_ten_to_result() {
        assert_eq!(add_ten_to_result(Ok(5)), Ok(15));
        assert_eq!(add_ten_to_result(Ok(0)), Ok(10));
        assert_eq!(
            add_ten_to_result(Err("error".to_string())),
            Err("error".to_string())
        );
        assert_eq!(add_ten_to_result(Ok(-10)), Ok(0));
    }

    #[test]
    fn test_filter_positive() {
        assert_eq!(filter_positive(Some(5)), Some(5));
        assert_eq!(filter_positive(Some(0)), None);
        assert_eq!(filter_positive(Some(-5)), None);
        assert_eq!(filter_positive(None), None);
        assert_eq!(filter_positive(Some(1)), Some(1));
    }

    #[test]
    fn test_result_or_minus_one() {
        assert_eq!(result_or_minus_one(Ok(42)), 42);
        assert_eq!(result_or_minus_one(Err("error".to_string())), -1);
        assert_eq!(result_or_minus_one(Ok(0)), 0);
        assert_eq!(result_or_minus_one(Ok(-1)), -1);
    }

    // Medium Tests
    #[test]
    fn test_parse_and_halve_if_even() {
        assert_eq!(parse_and_halve_if_even("10"), Some(5));
        assert_eq!(parse_and_halve_if_even("7"), None);
        assert_eq!(parse_and_halve_if_even("abc"), None);
        assert_eq!(parse_and_halve_if_even("0"), Some(0));
        assert_eq!(parse_and_halve_if_even("100"), Some(50));
        assert_eq!(parse_and_halve_if_even("-10"), Some(-5));
    }

    #[test]
    fn test_parse_and_divide() {
        assert_eq!(parse_and_divide("10", 2), Ok(5));
        assert_eq!(parse_and_divide("10", 3), Ok(3));
        assert!(parse_and_divide("abc", 2).is_err());
        assert!(parse_and_divide("10", 0).is_err());
        assert_eq!(parse_and_divide("100", 10), Ok(10));
    }

    #[test]
    fn test_first_or_second() {
        assert_eq!(first_or_second(Some(1), || Some(2)), Some(1));
        assert_eq!(first_or_second(None, || Some(2)), Some(2));
        assert_eq!(first_or_second(None, || None), None);
        assert_eq!(first_or_second(Some(0), || Some(10)), Some(0));
    }

    #[test]
    fn test_parse_with_custom_error() {
        assert_eq!(parse_with_custom_error("42"), Ok(42));
        assert_eq!(
            parse_with_custom_error("abc"),
            Err("Failed to parse 'abc'".to_string())
        );
        assert_eq!(parse_with_custom_error("0"), Ok(0));
        assert!(parse_with_custom_error("12.34").is_err());
    }

    #[test]
    fn test_parse_square_positive() {
        assert_eq!(parse_square_positive(Some("5")), Some(25));
        assert_eq!(parse_square_positive(Some("-5")), None);
        assert_eq!(parse_square_positive(Some("0")), None);
        assert_eq!(parse_square_positive(Some("abc")), None);
        assert_eq!(parse_square_positive(None), None);
        assert_eq!(parse_square_positive(Some("10")), Some(100));
    }

    #[test]
    fn test_first_result_or_second() {
        assert_eq!(first_result_or_second(Ok(1), Ok(2)), Ok(1));
        assert_eq!(
            first_result_or_second(Err("e1".to_string()), Ok(2)),
            Ok(2)
        );
        assert_eq!(
            first_result_or_second(Err("e1".to_string()), Err("e2".to_string())),
            Err("e2".to_string())
        );
        assert_eq!(first_result_or_second(Ok(0), Ok(1)), Ok(0));
    }

    #[test]
    fn test_transpose_option_result() {
        assert_eq!(transpose_option_result(Some(Ok(42))), Ok(Some(42)));
        assert_eq!(
            transpose_option_result(Some(Err("error".to_string()))),
            Err("error".to_string())
        );
        assert_eq!(transpose_option_result(None), Ok(None));
    }

    #[test]
    fn test_add_options() {
        assert_eq!(add_options(Some(3), Some(5)), Some(8));
        assert_eq!(add_options(None, Some(5)), None);
        assert_eq!(add_options(Some(3), None), None);
        assert_eq!(add_options(None, None), None);
        assert_eq!(add_options(Some(0), Some(0)), Some(0));
        assert_eq!(add_options(Some(-5), Some(5)), Some(0));
    }

    // Hard Tests
    #[test]
    fn test_validate_username() {
        assert_eq!(validate_username("john123"), Ok("john123".to_string()));
        assert_eq!(validate_username("abc"), Ok("abc".to_string()));
        assert!(validate_username("ab").is_err());
        assert!(validate_username("").is_err());
        assert!(validate_username("a".repeat(21).as_str()).is_err());
        assert!(validate_username("john@123").is_err());
        assert_eq!(
            validate_username("user_name_123"),
            Ok("user_name_123".to_string())
        );
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide("10", "2"), Ok(5.0));
        assert_eq!(safe_divide("10", "3"), Ok(10.0 / 3.0));
        assert!(safe_divide("abc", "2").is_err());
        assert!(safe_divide("10", "abc").is_err());
        assert!(safe_divide("10", "0").is_err());
        assert_eq!(safe_divide("0", "5"), Ok(0.0));
        assert_eq!(safe_divide("-10", "2"), Ok(-5.0));
    }

    #[test]
    fn test_parse_flexible() {
        assert_eq!(parse_flexible("42"), Some(42));
        assert_eq!(parse_flexible("  42  "), Some(42));
        assert_eq!(parse_flexible("42.7"), Some(42));
        assert_eq!(parse_flexible("abc"), None);
        assert_eq!(parse_flexible("0"), Some(0));
        assert_eq!(parse_flexible("-5"), Some(-5));
    }

    #[test]
    fn test_create_config() {
        assert_eq!(create_config("8080"), Ok(Config { port: 8080 }));
        assert_eq!(create_config("1024"), Ok(Config { port: 1024 }));
        assert!(create_config("1023").is_err());
        assert!(create_config("abc").is_err());
        assert_eq!(create_config("65535"), Ok(Config { port: 65535 }));
    }

    #[test]
    fn test_flatten_nested() {
        assert_eq!(flatten_nested(Some(Ok(Some(42)))), Ok(Some(42)));
        assert_eq!(flatten_nested(Some(Ok(None))), Ok(None));
        assert_eq!(
            flatten_nested(Some(Err("error".to_string()))),
            Err("error".to_string())
        );
        assert_eq!(flatten_nested(None), Ok(None));
    }

    // Expert Tests
    #[test]
    fn test_tap() {
        let mut log = Vec::new();
        let result = Some(5)
            .tap(|v| log.push(format!("Initial: {:?}", v)))
            .map(|n| n * 2)
            .tap(|v| log.push(format!("After map: {:?}", v)))
            .filter(|&n| n > 5)
            .tap(|v| log.push(format!("After filter: {:?}", v)));

        assert_eq!(result, Some(10));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn test_tap_with_result() {
        let mut called = false;
        let result: Result<i32, String> = Ok(42).tap(|_| called = true);
        assert_eq!(result, Ok(42));
        assert!(called);
    }

    #[test]
    fn test_create_validated_user() {
        let user = create_validated_user("john_doe", "john@example.com", 25);
        assert_eq!(
            user,
            Ok(User {
                username: "john_doe".to_string(),
                email: "john@example.com".to_string(),
                age: 25,
            })
        );

        // Invalid username
        assert!(create_validated_user("ab", "john@example.com", 25).is_err());

        // Invalid email
        assert!(create_validated_user("john_doe", "invalid", 25).is_err());

        // Invalid age
        assert!(create_validated_user("john_doe", "john@example.com", 10).is_err());

        // Multiple valid users
        let user2 = create_validated_user("alice123", "alice@test.com", 30);
        assert!(user2.is_ok());

        let user3 = create_validated_user("bob_smith", "bob@domain.org", 18);
        assert!(user3.is_ok());
    }

    #[test]
    fn test_create_user_collect_errors() {
        // Valid user
        let user = create_user_collect_errors("john_doe", "john@example.com", 25);
        assert!(user.is_ok());

        // Single error
        let result = create_user_collect_errors("ab", "john@example.com", 25);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
        }

        // Multiple errors
        let result = create_user_collect_errors("ab", "invalid", 10);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 3);
        }

        // All valid edge cases
        let user = create_user_collect_errors("abc", "a@b.c", 13);
        assert!(user.is_ok());

        let user = create_user_collect_errors("a".repeat(20).as_str(), "test@example.com", 120);
        assert!(user.is_ok());
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::InvalidUsername("test".to_string());
        assert_eq!(format!("{}", error), "Username error: test");

        let error = ValidationError::InvalidEmail("test".to_string());
        assert_eq!(format!("{}", error), "Email error: test");

        let error = ValidationError::InvalidAge("test".to_string());
        assert_eq!(format!("{}", error), "Age error: test");
    }

    // Additional edge case tests
    #[test]
    fn test_option_chaining_edge_cases() {
        // Test None propagation
        let result: Option<i32> = None.and_then(|x: i32| Some(x * 2)).map(|x| x + 1);
        assert_eq!(result, None);

        // Test Some chaining
        let result = Some(5).and_then(|x| Some(x * 2)).map(|x| x + 1);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_result_chaining_edge_cases() {
        // Test Err propagation
        let result: Result<i32, String> = Err::<i32, String>("initial".to_string())
            .and_then(|x| Ok(x * 2))
            .map(|x| x + 1);
        assert_eq!(result, Err("initial".to_string()));

        // Test Ok chaining
        let result: Result<i32, String> = Ok(5).and_then(|x| Ok(x * 2)).map(|x| x + 1);
        assert_eq!(result, Ok(11));
    }

    #[test]
    fn test_or_else_with_computation() {
        let result = None.or_else(|| Some(42));
        assert_eq!(result, Some(42));

        let result = Some(10).or_else(|| Some(42));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_map_err_chaining() {
        let result: Result<i32, String> = Err("error".to_string())
            .map_err(|e| format!("First: {}", e))
            .map_err(|e| format!("Second: {}", e));
        assert_eq!(result, Err("Second: First: error".to_string()));
    }
}
