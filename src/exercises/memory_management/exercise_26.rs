//! Exercise 26: Smart Drop - Complex cleanup scenarios
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement complex Drop logic
//! - Handle cleanup order and dependencies
//! - Prevent double-free and use-after-free

use std::cell::RefCell;
use std::rc::Rc;

pub struct Logger {
    logs: Rc<RefCell<Vec<String>>>,
}

impl Logger {
    pub fn new(logs: Rc<RefCell<Vec<String>>>) -> Self {
        logs.borrow_mut().push("Logger created".to_string());
        Logger { logs }
    }
    
    pub fn log(&self, message: &str) {
        self.logs.borrow_mut().push(message.to_string());
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.logs.borrow_mut().push("Logger dropped".to_string());
    }
}

pub struct Connection {
    id: u32,
    connected: bool,
}

impl Connection {
    pub fn new(id: u32) -> Self {
        Connection {
            id,
            connected: true,
        }
    }
    
    pub fn disconnect(&mut self) {
        self.connected = false;
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.connected {
            // Ensure we disconnect before dropping
            self.disconnect();
        }
    }
}

pub struct ConnectionPool {
    connections: Vec<Connection>,
}

impl ConnectionPool {
    pub fn new(size: usize) -> Self {
        let connections = (0..size)
            .map(|i| Connection::new(i as u32))
            .collect();
        ConnectionPool { connections }
    }
    
    pub fn get(&mut self, index: usize) -> Option<&mut Connection> {
        self.connections.get_mut(index)
    }
    
    pub fn size(&self) -> usize {
        self.connections.len()
    }
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        // Explicitly disconnect all connections
        for conn in &mut self.connections {
            if conn.is_connected() {
                conn.disconnect();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_lifecycle() {
        let logs = Rc::new(RefCell::new(Vec::new()));
        {
            let logger = Logger::new(logs.clone());
            logger.log("Test message");
        }
        
        let log_vec = logs.borrow();
        assert_eq!(log_vec.len(), 3);
        assert_eq!(log_vec[0], "Logger created");
        assert_eq!(log_vec[1], "Test message");
        assert_eq!(log_vec[2], "Logger dropped");
    }

    #[test]
    fn test_connection_drop() {
        let conn = Connection::new(1);
        assert!(conn.is_connected());
        drop(conn);
        // Connection should be disconnected on drop
    }

    #[test]
    fn test_connection_manual_disconnect() {
        let mut conn = Connection::new(2);
        assert!(conn.is_connected());
        conn.disconnect();
        assert!(!conn.is_connected());
    }

    #[test]
    fn test_connection_pool() {
        let mut pool = ConnectionPool::new(3);
        assert_eq!(pool.size(), 3);
        
        if let Some(conn) = pool.get(0) {
            assert!(conn.is_connected());
            assert_eq!(conn.id(), 0);
        }
    }

    #[test]
    fn test_connection_pool_drop() {
        let pool = ConnectionPool::new(5);
        assert_eq!(pool.size(), 5);
        drop(pool);
        // All connections should be properly cleaned up
    }

    #[test]
    fn test_multiple_loggers() {
        let logs = Rc::new(RefCell::new(Vec::new()));
        {
            let logger1 = Logger::new(logs.clone());
            logger1.log("Message 1");
            {
                let logger2 = Logger::new(logs.clone());
                logger2.log("Message 2");
            }
            logger1.log("Message 3");
        }
        
        let log_vec = logs.borrow();
        assert!(log_vec.contains(&"Logger created".to_string()));
        assert!(log_vec.contains(&"Logger dropped".to_string()));
    }
}
