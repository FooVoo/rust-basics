//! Exercise 22: Supertraits - Create complex trait hierarchies with requirements
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Define supertraits with multiple requirements
//! - Implement complex trait hierarchies
//! - Use trait bounds with supertraits

use std::fmt::{Debug, Display};

pub trait Entity: Debug + Clone {
    fn id(&self) -> u64;
}

pub trait Persistent: Entity {
    fn save(&self) -> String {
        todo!("Implement save")
    }
    
    fn load(id: u64) -> Self
    where
        Self: Sized;
}

pub trait Auditable: Persistent + Display {
    fn audit_log(&self) -> String {
        todo!("Implement audit_log")
    }
    
    fn last_modified(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub modified: u64,
}

impl Entity for User {
    fn id(&self) -> u64 {
        todo!("Implement id")
    }
}

impl Persistent for User {
    fn load(id: u64) -> Self {
        todo!("Implement load")
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Implement fmt")
    }
}

impl Auditable for User {
    fn last_modified(&self) -> u64 {
        todo!("Implement last_modified")
    }
}

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub modified: u64,
}

impl Entity for Product {
    fn id(&self) -> u64 {
        todo!("Implement id")
    }
}

impl Persistent for Product {
    fn load(id: u64) -> Self {
        todo!("Implement load")
    }
    
    fn save(&self) -> String {
        todo!("Implement save")
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Implement fmt")
    }
}

impl Auditable for Product {
    fn last_modified(&self) -> u64 {
        todo!("Implement last_modified")
    }
}

/// Function requiring Auditable (which requires Persistent and Display, which requires Entity)
pub fn full_audit<T: Auditable>(entity: &T) -> String {
    todo!("Implement full_audit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_entity() {
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            modified: 1000,
        };
        assert_eq!(user.id(), 1);
    }

    #[test]
    fn test_user_debug_clone() {
        let user = User {
            id: 2,
            name: "Bob".to_string(),
            modified: 2000,
        };
        let cloned = user.clone();
        assert_eq!(format!("{:?}", user), format!("{:?}", cloned));
    }

    #[test]
    fn test_user_persistent() {
        let user = User {
            id: 3,
            name: "Charlie".to_string(),
            modified: 3000,
        };
        let saved = user.save();
        assert!(saved.contains("Saving entity"));
        assert!(saved.contains("3"));
    }

    #[test]
    fn test_user_load() {
        let user = User::load(100);
        assert_eq!(user.id, 100);
        assert!(user.name.contains("100"));
    }

    #[test]
    fn test_user_display() {
        let user = User {
            id: 4,
            name: "Diana".to_string(),
            modified: 4000,
        };
        let display = format!("{}", user);
        assert!(display.contains("User(4)"));
        assert!(display.contains("Diana"));
    }

    #[test]
    fn test_user_auditable() {
        let user = User {
            id: 5,
            name: "Eve".to_string(),
            modified: 5000,
        };
        assert_eq!(user.last_modified(), 5000);
        let audit = user.audit_log();
        assert!(audit.contains("Audit:"));
    }

    #[test]
    fn test_product_entity() {
        let product = Product {
            id: 10,
            name: "Widget".to_string(),
            modified: 1500,
        };
        assert_eq!(product.id(), 10);
    }

    #[test]
    fn test_product_persistent_override() {
        let product = Product {
            id: 11,
            name: "Gadget".to_string(),
            modified: 1600,
        };
        let saved = product.save();
        assert!(saved.contains("Gadget"));
        assert!(saved.contains("database"));
    }

    #[test]
    fn test_full_audit_user() {
        let user = User {
            id: 6,
            name: "Frank".to_string(),
            modified: 6000,
        };
        let audit = full_audit(&user);
        assert!(audit.contains("Full Audit"));
        assert!(audit.contains("6"));
        assert!(audit.contains("Frank"));
        assert!(audit.contains("6000"));
    }

    #[test]
    fn test_full_audit_product() {
        let product = Product {
            id: 12,
            name: "Tool".to_string(),
            modified: 1700,
        };
        let audit = full_audit(&product);
        assert!(audit.contains("12"));
        assert!(audit.contains("Tool"));
        assert!(audit.contains("1700"));
    }

    #[test]
    fn test_trait_hierarchy() {
        let user = User {
            id: 7,
            name: "Grace".to_string(),
            modified: 7000,
        };
        
        // Can use all levels of the hierarchy
        let _id = user.id(); // Entity
        let _saved = user.save(); // Persistent
        let _display = format!("{}", user); // Display
        let _modified = user.last_modified(); // Auditable
        let _audit = user.audit_log(); // Auditable
        let _cloned = user.clone(); // Clone (required by Entity)
    }
}
