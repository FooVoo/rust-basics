//! Exercise 26: Resource Cleanup - Ensure cleanup on errors
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement RAII patterns
//! - Ensure cleanup happens on error paths
//! - Use Drop trait for resource management

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ResourceError {
    AlreadyAcquired,
    NotAcquired,
    OperationFailed(String),
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        todo!("Implement fmt")
    }
}

impl std::error::Error for ResourceError {}

/// A resource that tracks acquisition and release.
#[derive(Debug)]
pub struct TrackedResource {
    name: String,
    acquired: bool,
}

impl TrackedResource {
    pub fn new(name: impl Into<String>) -> Self  {
        todo!("A resource that tracks acquisition and release.")
    }
    
    pub fn acquire(&mut self) -> Result<(), ResourceError>  {
        todo!("Implement acquire")
    }
    
    pub fn release(&mut self) -> Result<(), ResourceError>  {
        todo!("Implement release")
    }
    
    pub fn is_acquired(&self) -> bool  {
        todo!("Implement is_acquired")
    }
}

/// RAII guard that ensures resource is released.
pub struct ResourceGuard<'a> {
    resource: &'a mut TrackedResource,
}

impl<'a> ResourceGuard<'a> {
    pub fn acquire(resource: &'a mut TrackedResource) -> Result<Self, ResourceError>  {
        todo!("RAII guard that ensures resource is released.")
    }
    
    pub fn perform_operation(&mut self, should_fail: bool) -> Result<(), ResourceError>  {
        todo!("Implement perform_operation")
    }
}

impl<'a> Drop for ResourceGuard<'a> {
    fn drop(&mut self)  {
        todo!("Implement drop")
    }
}

/// Perform operations with automatic cleanup.
pub fn with_resource<F, T>(
    resource: &mut TrackedResource,
    operation: F,
) -> Result<T, ResourceError>
where
    F: FnOnce(&mut ResourceGuard) -> Result<T, ResourceError>,
 {
    todo!("Perform operations with automatic cleanup.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_acquire_release() {
        let mut resource = TrackedResource::new("test");
        assert!(!resource.is_acquired());
        
        assert_eq!(resource.acquire(), Ok(()));
        assert!(resource.is_acquired());
        
        assert_eq!(resource.release(), Ok(()));
        assert!(!resource.is_acquired());
    }

    #[test]
    fn test_resource_double_acquire() {
        let mut resource = TrackedResource::new("test");
        resource.acquire().unwrap();
        
        assert_eq!(resource.acquire(), Err(ResourceError::AlreadyAcquired));
    }

    #[test]
    fn test_resource_release_without_acquire() {
        let mut resource = TrackedResource::new("test");
        assert_eq!(resource.release(), Err(ResourceError::NotAcquired));
    }

    #[test]
    fn test_guard_auto_cleanup() {
        let mut resource = TrackedResource::new("test");
        
        {
            let _guard = ResourceGuard::acquire(&mut resource).unwrap();
            // Can't check is_acquired here because of borrow checker
        }
        
        assert!(!resource.is_acquired());
    }

    #[test]
    fn test_guard_cleanup_on_error() {
        let mut resource = TrackedResource::new("test");
        
        let result = {
            let mut guard = ResourceGuard::acquire(&mut resource).unwrap();
            guard.perform_operation(true)
        };
        
        assert!(result.is_err());
        assert!(!resource.is_acquired());
    }

    #[test]
    fn test_with_resource_success() {
        let mut resource = TrackedResource::new("test");
        
        let result = with_resource(&mut resource, |guard| {
            guard.perform_operation(false)?;
            Ok(42)
        });
        
        assert_eq!(result, Ok(42));
        assert!(!resource.is_acquired());
    }

    #[test]
    fn test_with_resource_error() {
        let mut resource = TrackedResource::new("test");
        
        let result: Result<(), ResourceError> = with_resource(&mut resource, |guard| {
            guard.perform_operation(true)
        });
        
        assert!(result.is_err());
        assert!(!resource.is_acquired());
    }

    #[test]
    fn test_with_resource_panic_cleanup() {
        let mut resource = TrackedResource::new("test");
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _: Result<(), ResourceError> = with_resource(&mut resource, |_guard| {
                panic!("Test panic");
            });
        }));
        
        assert!(result.is_err());
        assert!(!resource.is_acquired());
    }

    #[test]
    fn test_nested_with_resource() {
        let mut resource1 = TrackedResource::new("res1");
        let mut resource2 = TrackedResource::new("res2");
        
        let result = with_resource(&mut resource1, |_guard1| {
            with_resource(&mut resource2, |_guard2| Ok(100))
        });
        
        assert_eq!(result, Ok(100));
        assert!(!resource1.is_acquired());
        assert!(!resource2.is_acquired());
    }
}
