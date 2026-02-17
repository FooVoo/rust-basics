//! Exercise 30: Type-State Pattern - Compile-Time State Machine
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement type-state pattern for compile-time guarantees
//! - Use phantom types to encode state in the type system
//! - Create APIs that are impossible to misuse
//! - Understand zero-cost abstractions with states

use std::marker::PhantomData;

/// State marker types
#[derive(Debug)]
pub struct Draft;
#[derive(Debug)]
pub struct PendingReview;
#[derive(Debug)]
pub struct Published;

/// Document with type-state pattern
#[derive(Debug)]
pub struct Document<State> {
    content: String,
    _state: PhantomData<State>,
}

// Methods available in Draft state
impl Document<Draft> {
    /// Creates a new draft document
    pub fn new() -> Self {
        Document {
            content: String::new(),
            _state: PhantomData,
        }
    }

    /// Creates a draft from existing content
    pub fn from_content(content: String) -> Self {
        Document {
            content,
            _state: PhantomData,
        }
    }

    /// Adds content to the draft
    pub fn add_content(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Submits the document for review
    pub fn submit_for_review(self) -> Document<PendingReview> {
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

// Methods available in PendingReview state
impl Document<PendingReview> {
    /// Approves the document and publishes it
    pub fn approve(self) -> Document<Published> {
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }

    /// Rejects the document and returns it to draft
    pub fn reject(self) -> Document<Draft> {
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }

    /// Gets content for review (read-only)
    pub fn preview(&self) -> &str {
        &self.content
    }
}

// Methods available in Published state
impl Document<Published> {
    /// Gets the published content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Archives the published document (returns to draft for revision)
    pub fn archive(self) -> Document<Draft> {
        Document {
            content: self.content,
            _state: PhantomData,
        }
    }
}

/// Connection builder with type-state pattern
#[derive(Debug)]
pub struct NoHost;
#[derive(Debug)]
pub struct HasHost;
#[derive(Debug)]
pub struct NoAuth;
#[derive(Debug)]
pub struct HasAuth;

#[derive(Debug)]
pub struct ConnectionBuilder<Host, Auth> {
    host: Option<String>,
    username: Option<String>,
    password: Option<String>,
    _host_state: PhantomData<Host>,
    _auth_state: PhantomData<Auth>,
}

impl ConnectionBuilder<NoHost, NoAuth> {
    /// Creates a new connection builder
    pub fn new() -> Self {
        ConnectionBuilder {
            host: None,
            username: None,
            password: None,
            _host_state: PhantomData,
            _auth_state: PhantomData,
        }
    }
}

impl<Auth> ConnectionBuilder<NoHost, Auth> {
    /// Sets the host
    pub fn host(self, host: String) -> ConnectionBuilder<HasHost, Auth> {
        ConnectionBuilder {
            host: Some(host),
            username: self.username,
            password: self.password,
            _host_state: PhantomData,
            _auth_state: PhantomData,
        }
    }
}

impl<Host> ConnectionBuilder<Host, NoAuth> {
    /// Sets authentication credentials
    pub fn auth(self, username: String, password: String) -> ConnectionBuilder<Host, HasAuth> {
        ConnectionBuilder {
            host: self.host,
            username: Some(username),
            password: Some(password),
            _host_state: PhantomData,
            _auth_state: PhantomData,
        }
    }
}

impl ConnectionBuilder<HasHost, HasAuth> {
    /// Builds the connection (only available when host and auth are set)
    pub fn build(self) -> Connection {
        Connection {
            host: self.host.unwrap(),
            username: self.username.unwrap(),
            password: self.password.unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Connection {
    host: String,
    username: String,
    password: String,
}

impl Connection {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_workflow() {
        let mut doc = Document::new();
        doc.add_content("Hello, ");
        doc.add_content("World!");

        let doc = doc.submit_for_review();
        assert_eq!(doc.preview(), "Hello, World!");

        let doc = doc.approve();
        assert_eq!(doc.content(), "Hello, World!");
    }

    #[test]
    fn test_document_rejection() {
        let mut doc = Document::new();
        doc.add_content("Initial content");

        let doc = doc.submit_for_review();
        let mut doc = doc.reject();

        doc.add_content(" - revised");
        let doc = doc.submit_for_review();
        let doc = doc.approve();
        assert_eq!(doc.content(), "Initial content - revised");
    }

    #[test]
    fn test_document_archive() {
        let doc = Document::from_content("Published content".to_string());
        let doc = doc.submit_for_review();
        let doc = doc.approve();
        let mut doc = doc.archive();
        doc.add_content(" - updated");
        let doc = doc.submit_for_review();
        assert_eq!(doc.preview(), "Published content - updated");
    }

    #[test]
    fn test_connection_builder() {
        let conn = ConnectionBuilder::new()
            .host("localhost".to_string())
            .auth("admin".to_string(), "secret".to_string())
            .build();

        assert_eq!(conn.host(), "localhost");
        assert_eq!(conn.username(), "admin");
    }

    #[test]
    fn test_connection_builder_different_order() {
        let conn = ConnectionBuilder::new()
            .auth("admin".to_string(), "secret".to_string())
            .host("localhost".to_string())
            .build();

        assert_eq!(conn.host(), "localhost");
        assert_eq!(conn.username(), "admin");
    }

    // The following would not compile (which is the point!):
    // #[test]
    // fn test_cannot_build_without_host() {
    //     let conn = ConnectionBuilder::new()
    //         .auth("admin".to_string(), "secret".to_string())
    //         .build(); // Error: build() not available without host
    // }

    // #[test]
    // fn test_cannot_modify_published() {
    //     let mut doc = Document::from_content("content".to_string());
    //     let doc = doc.submit_for_review();
    //     let mut doc = doc.approve();
    //     doc.add_content("more"); // Error: add_content() not available in Published state
    // }
}
