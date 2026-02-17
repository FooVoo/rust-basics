//! Exercise 19: Enum State Transitions - Basic State Machine
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Model state machines with enums
//! - Implement state transitions
//! - Validate state changes

#[derive(Debug, PartialEq, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

impl ConnectionState {
    /// Attempts to transition to the Connecting state
    pub fn connect(&mut self) -> Result<(), String> {
        match self {
            ConnectionState::Disconnected => {
                *self = ConnectionState::Connecting;
                Ok(())
            }
            ConnectionState::Error(_) => {
                *self = ConnectionState::Connecting;
                Ok(())
            }
            _ => Err("Already connecting or connected".to_string()),
        }
    }

    /// Completes the connection
    pub fn complete_connection(&mut self) -> Result<(), String> {
        match self {
            ConnectionState::Connecting => {
                *self = ConnectionState::Connected;
                Ok(())
            }
            _ => Err("Not in connecting state".to_string()),
        }
    }

    /// Disconnects
    pub fn disconnect(&mut self) {
        *self = ConnectionState::Disconnected;
    }

    /// Sets error state
    pub fn set_error(&mut self, msg: String) {
        *self = ConnectionState::Error(msg);
    }

    /// Returns true if connected
    pub fn is_connected(&self) -> bool {
        matches!(self, ConnectionState::Connected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_flow() {
        let mut conn = ConnectionState::Disconnected;
        assert!(conn.connect().is_ok());
        assert_eq!(conn, ConnectionState::Connecting);
        assert!(conn.complete_connection().is_ok());
        assert_eq!(conn, ConnectionState::Connected);
        assert!(conn.is_connected());
    }

    #[test]
    fn test_invalid_transitions() {
        let mut conn = ConnectionState::Connected;
        assert!(conn.connect().is_err());
        assert!(conn.complete_connection().is_err());
    }

    #[test]
    fn test_disconnect() {
        let mut conn = ConnectionState::Connected;
        conn.disconnect();
        assert_eq!(conn, ConnectionState::Disconnected);
    }

    #[test]
    fn test_error_recovery() {
        let mut conn = ConnectionState::Error("timeout".to_string());
        assert!(conn.connect().is_ok());
        assert_eq!(conn, ConnectionState::Connecting);
    }
}
