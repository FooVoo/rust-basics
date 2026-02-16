//! Exercise 28: Generic Type-State Pattern - Implement type-state with generics
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use phantom types for type-state pattern
//! - Ensure correct state transitions at compile time
//! - Build state machines with zero runtime cost

use std::marker::PhantomData;

// State markers
pub struct Draft;
pub struct Review;
pub struct Published;

/// A generic document with type-state pattern.
pub struct Document<State> {
    content: String,
    _state: PhantomData<State>,
}

impl Document<Draft> {
    /// Creates a new draft document.
    pub fn new(content: String) -> Self {
        Document {
            content,
            _state: PhantomData,
        }
    }

    /// Edits the content (only available in Draft state).
    pub fn edit(&mut self, new_content: String) {
        self.content = new_content;
    }

    /// Submits the document for review.
    pub fn submit_for_review(self) -> Document<Review> {
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

impl Document<Review> {
    /// Approves the document for publishing.
    pub fn approve(self) -> Document<Published> {
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }

    /// Rejects the document back to draft.
    pub fn reject(self) -> Document<Draft> {
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }

    /// Gets the content for review.
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Document<Published> {
    /// Gets the published content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Archives the document (ends its lifecycle).
    pub fn archive(self) -> String {
        self.content
    }
}

// Connection states
pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

/// A generic connection with type-state pattern.
pub struct Connection<State> {
    host: String,
    port: u16,
    _state: PhantomData<State>,
}

impl Connection<Disconnected> {
    /// Creates a new disconnected connection.
    pub fn new(host: String, port: u16) -> Self {
        Connection {
            host,
            port,
            _state: PhantomData,
        }
    }

    /// Connects to the server.
    pub fn connect(self) -> Result<Connection<Connected>, String> {
        if self.host.is_empty() {
            return Err("Invalid host".to_string());
        }
        Ok(Connection {
            host: self.host,
            port: self.port,
            _state: PhantomData,
        })
    }
}

impl Connection<Connected> {
    /// Authenticates the connection.
    pub fn authenticate(self, _password: &str) -> Result<Connection<Authenticated>, String> {
        // In a real scenario, would check password
        Ok(Connection {
            host: self.host,
            port: self.port,
            _state: PhantomData,
        })
    }

    /// Disconnects from the server.
    pub fn disconnect(self) -> Connection<Disconnected> {
        Connection {
            host: self.host,
            port: self.port,
            _state: PhantomData,
        }
    }

    /// Gets the connection info.
    pub fn info(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Connection<Authenticated> {
    /// Sends data (only available when authenticated).
    pub fn send(&self, data: &str) -> Result<(), String> {
        if data.is_empty() {
            return Err("Cannot send empty data".to_string());
        }
        Ok(())
    }

    /// Logs out and returns to connected state.
    pub fn logout(self) -> Connection<Connected> {
        Connection {
            host: self.host,
            port: self.port,
            _state: PhantomData,
        }
    }

    /// Disconnects and logs out.
    pub fn disconnect(self) -> Connection<Disconnected> {
        Connection {
            host: self.host,
            port: self.port,
            _state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_draft() {
        let mut doc = Document::<Draft>::new("Initial content".to_string());
        doc.edit("Updated content".to_string());
    }

    #[test]
    fn test_document_workflow() {
        let doc = Document::<Draft>::new("Content".to_string());
        let doc = doc.submit_for_review();
        let doc = doc.approve();
        assert_eq!(doc.content(), "Content");
    }

    #[test]
    fn test_document_rejection() {
        let doc = Document::<Draft>::new("Content".to_string());
        let doc = doc.submit_for_review();
        let mut doc = doc.reject();
        doc.edit("Revised content".to_string());
    }

    #[test]
    fn test_document_archive() {
        let doc = Document::<Draft>::new("Content".to_string());
        let doc = doc.submit_for_review();
        let doc = doc.approve();
        let archived = doc.archive();
        assert_eq!(archived, "Content");
    }

    #[test]
    fn test_connection_new() {
        let conn = Connection::<Disconnected>::new("localhost".to_string(), 8080);
        let _conn = conn.connect().unwrap();
    }

    #[test]
    fn test_connection_authenticate() {
        let conn = Connection::<Disconnected>::new("localhost".to_string(), 8080);
        let conn = conn.connect().unwrap();
        let _conn = conn.authenticate("password").unwrap();
    }

    #[test]
    fn test_connection_send() {
        let conn = Connection::<Disconnected>::new("localhost".to_string(), 8080);
        let conn = conn.connect().unwrap();
        let conn = conn.authenticate("password").unwrap();
        assert!(conn.send("data").is_ok());
    }

    #[test]
    fn test_connection_logout() {
        let conn = Connection::<Disconnected>::new("localhost".to_string(), 8080);
        let conn = conn.connect().unwrap();
        let conn = conn.authenticate("password").unwrap();
        let _conn = conn.logout();
    }

    #[test]
    fn test_connection_info() {
        let conn = Connection::<Disconnected>::new("localhost".to_string(), 8080);
        let conn = conn.connect().unwrap();
        assert_eq!(conn.info(), "localhost:8080");
    }

    #[test]
    fn test_connection_full_cycle() {
        let conn = Connection::<Disconnected>::new("localhost".to_string(), 8080);
        let conn = conn.connect().unwrap();
        let conn = conn.authenticate("password").unwrap();
        let conn = conn.logout();
        let _conn = conn.disconnect();
    }
}
