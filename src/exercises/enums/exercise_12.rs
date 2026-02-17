//! Exercise 12: Pattern Matching Guards - Using if in Patterns
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Use pattern guards (if conditions) in match arms
//! - Combine patterns with boolean conditions
//! - Extract and validate data simultaneously

#[derive(Debug, PartialEq, Clone)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    /// Returns a description of the temperature
    pub fn describe(&self) -> &'static str {
        match self {
            Temperature::Celsius(c) if *c < 0.0 => "freezing",
            Temperature::Celsius(c) if *c < 20.0 => "cold",
            Temperature::Celsius(c) if *c < 30.0 => "comfortable",
            Temperature::Celsius(_) => "hot",
            Temperature::Fahrenheit(f) if *f < 32.0 => "freezing",
            Temperature::Fahrenheit(f) if *f < 68.0 => "cold",
            Temperature::Fahrenheit(f) if *f < 86.0 => "comfortable",
            Temperature::Fahrenheit(_) => "hot",
        }
    }

    /// Converts to Celsius
    pub fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
        }
    }
}

/// Returns true if the value is positive and even
pub fn is_positive_even(opt: Option<i32>) -> bool {
    matches!(opt, Some(n) if n > 0 && n % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_celsius() {
        assert_eq!(Temperature::Celsius(-5.0).describe(), "freezing");
        assert_eq!(Temperature::Celsius(10.0).describe(), "cold");
        assert_eq!(Temperature::Celsius(25.0).describe(), "comfortable");
        assert_eq!(Temperature::Celsius(35.0).describe(), "hot");
    }

    #[test]
    fn test_describe_fahrenheit() {
        assert_eq!(Temperature::Fahrenheit(20.0).describe(), "freezing");
        assert_eq!(Temperature::Fahrenheit(50.0).describe(), "cold");
        assert_eq!(Temperature::Fahrenheit(75.0).describe(), "comfortable");
        assert_eq!(Temperature::Fahrenheit(95.0).describe(), "hot");
    }

    #[test]
    fn test_to_celsius() {
        assert_eq!(Temperature::Celsius(25.0).to_celsius(), 25.0);
        assert!((Temperature::Fahrenheit(32.0).to_celsius() - 0.0).abs() < 0.01);
        assert!((Temperature::Fahrenheit(212.0).to_celsius() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_is_positive_even() {
        assert!(is_positive_even(Some(2)));
        assert!(is_positive_even(Some(100)));
        assert!(!is_positive_even(Some(1)));
        assert!(!is_positive_even(Some(-2)));
        assert!(!is_positive_even(None));
    }
}
