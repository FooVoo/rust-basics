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
    pub fn new(logs: Rc<RefCell<Vec<String>>>) -> Self  {
        todo!("Implement new")
    }
    
    pub fn log(&self, message: &str)  {
        todo!("Implement log")
    }
}

impl Drop for Logger {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

pub struct Connection {
    id: u32,
    connected: bool,
}

impl Connection {
    pub fn new(id: u32) -> Self  {
        todo!("Implement new")
    }
    
    pub fn disconnect(&mut self)  {
        todo!("Implement disconnect")
    }
    
    pub fn is_connected(&self) -> bool  {
        todo!("Implement is_connected")
    }
    
    pub fn id(&self) -> u32  {
        todo!("Implement id")
    }
}

impl Drop for Connection {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

pub struct ConnectionPool {
    connections: Vec<Connection>,
}

impl ConnectionPool {
    pub fn new(size: usize) -> Self  {
        todo!("Implement new")
    }
    
    pub fn get(&mut self, index: usize) -> Option<&mut Connection>  {
        todo!("Implement get")
    }
    
    pub fn size(&self) -> usize  {
        todo!("Implement size")
    }
}

impl Drop for ConnectionPool {
    fn drop(&mut self)  {
        todo!("Implement drop")
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
