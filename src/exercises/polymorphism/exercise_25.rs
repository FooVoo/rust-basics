//! Exercise 25: Object Safety - Understand and work with object-safe traits
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand object safety rules
//! - Know which traits can be trait objects
//! - Work around object safety limitations

pub trait ObjectSafe {
    fn process(&self) -> String;
    fn get_value(&self) -> i32;
}

// This trait is object-safe: can be used as dyn Trait
pub struct SafeImpl {
    value: i32,
}

impl ObjectSafe for SafeImpl {
    fn process(&self) -> String  {
        todo!("Implement process")
    }
    
    fn get_value(&self) -> i32  {
        todo!("Implement get_value")
    }
}

pub fn use_as_trait_object(obj: &dyn ObjectSafe) -> String  {
    todo!("Implement use_as_trait_object")
}

// Trait with generic method - NOT object safe
pub trait NotObjectSafe {
    fn process<T>(&self, value: T) -> String
    where
        T: std::fmt::Display;
}

// Workaround: Remove generics to make it object-safe
pub trait ObjectSafeAlternative {
    fn process_string(&self, value: &str) -> String;
    fn process_i32(&self, value: i32) -> String;
}

pub struct AlternativeImpl;

impl ObjectSafeAlternative for AlternativeImpl {
    fn process_string(&self, value: &str) -> String  {
        todo!("Implement process_string")
    }
    
    fn process_i32(&self, value: i32) -> String  {
        todo!("Implement process_i32")
    }
}

// Trait without self parameter - NOT object safe
// pub trait NotObjectSafeNoSelf {
//     fn create() -> Self;
// }

// Workaround: Use Box<Self>
pub trait ObjectSafeFactory {
    fn create_boxed() -> Box<Self>
    where
        Self: Sized;
    
    fn describe(&self) -> String;
}

pub struct FactoryImpl {
    name: String,
}

impl ObjectSafeFactory for FactoryImpl {
    fn create_boxed() -> Box<Self>  {
        todo!("Implement create_boxed")
    }
    
    fn describe(&self) -> String  {
        todo!("Implement describe")
    }
}

// Trait with Sized bound - handle carefully
pub trait MaybeSized {
    fn get_data(&self) -> String;
}

impl MaybeSized for str {
    fn get_data(&self) -> String  {
        todo!("Implement get_data")
    }
}

impl MaybeSized for String {
    fn get_data(&self) -> String  {
        todo!("Implement get_data")
    }
}

pub fn use_maybe_sized(obj: &dyn MaybeSized) -> String  {
    todo!("Implement use_maybe_sized")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_safe_trait_object() {
        let safe = SafeImpl { value: 42 };
        let obj: &dyn ObjectSafe = &safe;
        
        assert_eq!(obj.get_value(), 42);
        assert!(obj.process().contains("Processing"));
    }

    #[test]
    fn test_use_as_trait_object() {
        let safe = SafeImpl { value: 100 };
        let result = use_as_trait_object(&safe);
        
        assert!(result.contains("Processing"));
        assert!(result.contains("100"));
    }

    #[test]
    fn test_object_safe_alternative() {
        let alt = AlternativeImpl;
        let obj: &dyn ObjectSafeAlternative = &alt;
        
        assert_eq!(obj.process_string("test"), "String: test");
        assert_eq!(obj.process_i32(42), "i32: 42");
    }

    #[test]
    fn test_collection_of_trait_objects() {
        let objs: Vec<Box<dyn ObjectSafe>> = vec![
            Box::new(SafeImpl { value: 1 }),
            Box::new(SafeImpl { value: 2 }),
            Box::new(SafeImpl { value: 3 }),
        ];
        
        let values: Vec<i32> = objs.iter().map(|o| o.get_value()).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_factory_create() {
        let instance = FactoryImpl::create_boxed();
        assert!(instance.describe().contains("Factory"));
    }

    #[test]
    fn test_factory_describe() {
        let instance = FactoryImpl {
            name: "Test".to_string(),
        };
        // Can use describe through trait object
        let obj: &dyn ObjectSafeFactory = &instance;
        assert!(obj.describe().contains("Test"));
    }

    #[test]
    fn test_maybe_sized_str() {
        let s = "hello".to_string();
        let result = use_maybe_sized(&s);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_maybe_sized_string() {
        let s = String::from("world");
        let result = use_maybe_sized(&s);
        assert_eq!(result, "world");
    }

    #[test]
    fn test_object_safety_rules() {
        // Object-safe traits can be used as trait objects
        let safe1 = SafeImpl { value: 10 };
        let safe2 = SafeImpl { value: 20 };
        
        let objects: Vec<&dyn ObjectSafe> = vec![&safe1, &safe2];
        
        for obj in objects {
            assert!(obj.get_value() > 0);
        }
    }

    #[test]
    fn test_workaround_patterns() {
        // Instead of generic method, use specific methods
        let alt = AlternativeImpl;
        
        let result1 = alt.process_string("test");
        let result2 = alt.process_i32(42);
        
        assert!(result1.contains("String"));
        assert!(result2.contains("i32"));
    }

    #[test]
    fn test_mixed_trait_objects() {
        let safe = SafeImpl { value: 50 };
        let alt = AlternativeImpl;
        
        // Different object-safe traits
        let obj1: &dyn ObjectSafe = &safe;
        let obj2: &dyn ObjectSafeAlternative = &alt;
        
        assert_eq!(obj1.get_value(), 50);
        assert!(obj2.process_string("test").contains("String"));
    }

    #[test]
    fn test_boxed_trait_objects() {
        let objects: Vec<Box<dyn ObjectSafe>> = vec![
            Box::new(SafeImpl { value: 1 }),
            Box::new(SafeImpl { value: 2 }),
        ];
        
        let sum: i32 = objects.iter().map(|o| o.get_value()).sum();
        assert_eq!(sum, 3);
    }
}
