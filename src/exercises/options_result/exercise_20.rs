//! Exercise 20: Combining multiple Results with ?
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use ? operator with multiple Results
//! - Handle different error types
//! - Build complex operations with early returns

/// Parse two numbers and compute their average.
pub fn average_from_strings(a: &str, b: &str) -> Result<f64, String>  {
    todo!("Parse two numbers and compute their average.")
}

/// Parse and perform a calculation (a + b) * c.
pub fn calculate(a: &str, b: &str, c: &str) -> Result<i32, String>  {
    todo!("Parse and perform a calculation (a + b) * c.")
}

/// Get element from slice by index and parse it.
pub fn get_and_parse(strings: &[&str], index: usize) -> Result<i32, String>  {
    todo!("Get element from slice by index and parse it.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_from_strings() {
        assert_eq!(average_from_strings("10", "20"), Ok(15.0));
        assert_eq!(average_from_strings("5", "15"), Ok(10.0));
        assert!(average_from_strings("abc", "10").is_err());
        assert!(average_from_strings("10", "xyz").is_err());
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate("2", "3", "4"), Ok(20)); // (2+3)*4
        assert_eq!(calculate("1", "1", "5"), Ok(10)); // (1+1)*5
        assert!(calculate("abc", "2", "3").is_err());
    }

    #[test]
    fn test_get_and_parse() {
        assert_eq!(get_and_parse(&["1", "2", "3"], 1), Ok(2));
        assert!(get_and_parse(&["1", "2"], 5).is_err());
        assert!(get_and_parse(&["abc"], 0).is_err());
    }
}
