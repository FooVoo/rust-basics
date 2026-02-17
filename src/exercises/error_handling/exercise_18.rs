//! Exercise 18: Configuration Parsing - Parse configuration with defaults
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Parse configuration with optional fields
//! - Provide default values
//! - Validate configuration

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout: u32,
    pub max_connections: u32,
}

impl Default for Config {
    fn default() -> Self {
        todo!("Implement default")
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    InvalidPort(String),
    InvalidTimeout(String),
    InvalidMaxConnections(String),
    EmptyHost,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!("Implement fmt")
    }
}

impl std::error::Error for ConfigError {}

/// Parse configuration from key-value map with defaults.
pub fn parse_config(data: &HashMap<String, String>) -> Result<Config, ConfigError> {
    todo!("Implement parse_config")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_empty() {
        let data = HashMap::new();
        let config = parse_config(&data).unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_parse_config_custom_host() {
        let mut data = HashMap::new();
        data.insert("host".to_string(), "example.com".to_string());
        
        let config = parse_config(&data).unwrap();
        assert_eq!(config.host, "example.com");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_parse_config_custom_port() {
        let mut data = HashMap::new();
        data.insert("port".to_string(), "3000".to_string());
        
        let config = parse_config(&data).unwrap();
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_parse_config_all_custom() {
        let mut data = HashMap::new();
        data.insert("host".to_string(), "api.example.com".to_string());
        data.insert("port".to_string(), "443".to_string());
        data.insert("timeout".to_string(), "60".to_string());
        data.insert("max_connections".to_string(), "200".to_string());
        
        let config = parse_config(&data).unwrap();
        assert_eq!(config.host, "api.example.com");
        assert_eq!(config.port, 443);
        assert_eq!(config.timeout, 60);
        assert_eq!(config.max_connections, 200);
    }

    #[test]
    fn test_parse_config_empty_host() {
        let mut data = HashMap::new();
        data.insert("host".to_string(), "".to_string());
        
        assert_eq!(parse_config(&data), Err(ConfigError::EmptyHost));
    }

    #[test]
    fn test_parse_config_invalid_port() {
        let mut data = HashMap::new();
        data.insert("port".to_string(), "abc".to_string());
        
        assert!(matches!(parse_config(&data), Err(ConfigError::InvalidPort(_))));
    }

    #[test]
    fn test_parse_config_zero_port() {
        let mut data = HashMap::new();
        data.insert("port".to_string(), "0".to_string());
        
        assert!(matches!(parse_config(&data), Err(ConfigError::InvalidPort(_))));
    }

    #[test]
    fn test_parse_config_invalid_timeout() {
        let mut data = HashMap::new();
        data.insert("timeout".to_string(), "xyz".to_string());
        
        assert!(matches!(parse_config(&data), Err(ConfigError::InvalidTimeout(_))));
    }

    #[test]
    fn test_parse_config_zero_timeout() {
        let mut data = HashMap::new();
        data.insert("timeout".to_string(), "0".to_string());
        
        assert!(matches!(parse_config(&data), Err(ConfigError::InvalidTimeout(_))));
    }
}
