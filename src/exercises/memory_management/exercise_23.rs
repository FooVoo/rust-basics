//! Exercise 23: RAII Pattern - Resource Acquisition Is Initialization
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Implement RAII for resource management
//! - Ensure proper cleanup with Drop
//! - Prevent resource leaks

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub struct TempFile {
    path: String,
    file: Option<File>,
}

impl TempFile {
    pub fn new(path: String) -> io::Result<Self> {
        let file = File::create(&path)?;
        Ok(TempFile {
            path,
            file: Some(file),
        })
    }
    
    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.write_all(data)?;
        }
        Ok(())
    }
    
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Close file first
        self.file.take();
        // Then remove it
        let _ = std::fs::remove_file(&self.path);
    }
}

pub struct Guard<'a, T> {
    data: &'a mut T,
    cleanup_fn: Box<dyn FnOnce(&mut T) + 'a>,
}

impl<'a, T> Guard<'a, T> {
    pub fn new(data: &'a mut T, cleanup_fn: impl FnOnce(&mut T) + 'a) -> Self {
        Guard {
            data,
            cleanup_fn: Box::new(cleanup_fn),
        }
    }
    
    pub fn get(&self) -> &T {
        self.data
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        // Safety: cleanup_fn is taken only once during drop
        let cleanup = std::mem::replace(&mut self.cleanup_fn, Box::new(|_| {}));
        cleanup(self.data);
    }
}

/// Create a guard that resets a value to zero on drop.
pub fn create_reset_guard(value: &mut i32) -> Guard<'_, i32> {
    Guard::new(value, |v| *v = 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_file_creation() {
        let path = "/tmp/test_file_creation.txt";
        {
            let temp = TempFile::new(path.to_string()).unwrap();
            assert_eq!(temp.path(), path);
            assert!(Path::new(path).exists());
        }
        // File should be removed after temp is dropped
        assert!(!Path::new(path).exists());
    }

    #[test]
    fn test_temp_file_write() {
        let path = "/tmp/test_file_write.txt";
        {
            let mut temp = TempFile::new(path.to_string()).unwrap();
            temp.write(b"test data").unwrap();
        }
        assert!(!Path::new(path).exists());
    }

    #[test]
    fn test_guard_reset() {
        let mut value = 42;
        {
            let mut guard = create_reset_guard(&mut value);
            *guard.get_mut() = 100;
            assert_eq!(*guard.get(), 100);
        }
        // Value should be reset to 0 after guard is dropped
        assert_eq!(value, 0);
    }

    #[test]
    fn test_guard_cleanup() {
        let mut data = vec![1, 2, 3];
        {
            let _guard = Guard::new(&mut data, |v| v.clear());
            // Cannot check data.len() here - it's mutably borrowed
        }
        // Vector should be cleared after guard is dropped
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_nested_guards() {
        let mut x = 10;
        let mut y = 5;
        {
            let _g1 = Guard::new(&mut x, |v| *v += 1);
            {
                let mut g2 = Guard::new(&mut y, |v| *v *= 2);
                *g2.get_mut() = 20;
            }
            // g2 dropped: 20 * 2 = 40
            assert_eq!(y, 40);
        }
        // g1 dropped: 10 + 1 = 11
        assert_eq!(x, 11);
    }
}
