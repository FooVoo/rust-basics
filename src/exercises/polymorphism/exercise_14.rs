//! Exercise 14: Trait Objects in Collections - Store different types using trait objects
//! Difficulty: Medium
//!
//! # Learning Objectives
//! - Create collections of trait objects
//! - Work with heterogeneous data
//! - Understand limitations of trait objects

pub trait Task {
    fn execute(&self) -> String;
    fn priority(&self) -> u32;
}

pub struct EmailTask {
    pub to: String,
    pub subject: String,
    pub priority: u32,
}

impl Task for EmailTask {
    fn execute(&self) -> String {
        todo!("Implement execute")
    }
    
    fn priority(&self) -> u32 {
        todo!("Implement priority")
    }
}

pub struct FileTask {
    pub filename: String,
    pub operation: String,
    pub priority: u32,
}

impl Task for FileTask {
    fn execute(&self) -> String {
        todo!("Implement execute")
    }
    
    fn priority(&self) -> u32 {
        todo!("Implement priority")
    }
}

pub struct DatabaseTask {
    pub query: String,
    pub priority: u32,
}

impl Task for DatabaseTask {
    fn execute(&self) -> String {
        todo!("Implement execute")
    }
    
    fn priority(&self) -> u32 {
        todo!("Implement priority")
    }
}

/// Task queue holding different task types
pub struct TaskQueue {
    tasks: Vec<Box<dyn Task>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        todo!("Implement new")
    }
    
    pub fn add_task(&mut self, task: Box<dyn Task>) {
        todo!("Implement add_task")
    }
    
    pub fn execute_all(&self) -> Vec<String> {
        todo!("Implement execute_all")
    }
    
    pub fn execute_by_priority(&self) -> Vec<String> {
        todo!("Implement execute_by_priority")
    }
    
    pub fn count(&self) -> usize {
        todo!("Implement count")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_task() {
        let task = EmailTask {
            to: "test@example.com".to_string(),
            subject: "Hello".to_string(),
            priority: 5,
        };
        assert!(task.execute().contains("test@example.com"));
        assert_eq!(task.priority(), 5);
    }

    #[test]
    fn test_file_task() {
        let task = FileTask {
            filename: "data.txt".to_string(),
            operation: "delete".to_string(),
            priority: 3,
        };
        assert!(task.execute().contains("data.txt"));
        assert_eq!(task.priority(), 3);
    }

    #[test]
    fn test_database_task() {
        let task = DatabaseTask {
            query: "SELECT * FROM users".to_string(),
            priority: 8,
        };
        assert!(task.execute().contains("SELECT"));
        assert_eq!(task.priority(), 8);
    }

    #[test]
    fn test_task_queue_creation() {
        let queue = TaskQueue::new();
        assert_eq!(queue.count(), 0);
    }

    #[test]
    fn test_task_queue_add() {
        let mut queue = TaskQueue::new();
        queue.add_task(Box::new(EmailTask {
            to: "user@test.com".to_string(),
            subject: "Test".to_string(),
            priority: 1,
        }));
        assert_eq!(queue.count(), 1);
    }

    #[test]
    fn test_task_queue_execute_all() {
        let mut queue = TaskQueue::new();
        queue.add_task(Box::new(EmailTask {
            to: "a@test.com".to_string(),
            subject: "A".to_string(),
            priority: 2,
        }));
        queue.add_task(Box::new(FileTask {
            filename: "file.txt".to_string(),
            operation: "read".to_string(),
            priority: 3,
        }));
        
        let results = queue.execute_all();
        assert_eq!(results.len(), 2);
        assert!(results[0].contains("a@test.com"));
        assert!(results[1].contains("file.txt"));
    }

    #[test]
    fn test_task_queue_priority() {
        let mut queue = TaskQueue::new();
        queue.add_task(Box::new(EmailTask {
            to: "low@test.com".to_string(),
            subject: "Low".to_string(),
            priority: 1,
        }));
        queue.add_task(Box::new(FileTask {
            filename: "high.txt".to_string(),
            operation: "write".to_string(),
            priority: 9,
        }));
        queue.add_task(Box::new(DatabaseTask {
            query: "UPDATE".to_string(),
            priority: 5,
        }));
        
        let results = queue.execute_by_priority();
        assert_eq!(results.len(), 3);
        assert!(results[0].contains("high.txt")); // Priority 9
        assert!(results[1].contains("UPDATE"));    // Priority 5
        assert!(results[2].contains("low@test.com")); // Priority 1
    }

    #[test]
    fn test_heterogeneous_collection() {
        let tasks: Vec<Box<dyn Task>> = vec![
            Box::new(EmailTask {
                to: "test@test.com".to_string(),
                subject: "Subject".to_string(),
                priority: 1,
            }),
            Box::new(FileTask {
                filename: "test.txt".to_string(),
                operation: "delete".to_string(),
                priority: 2,
            }),
            Box::new(DatabaseTask {
                query: "SELECT 1".to_string(),
                priority: 3,
            }),
        ];
        
        assert_eq!(tasks.len(), 3);
        for task in &tasks {
            assert!(!task.execute().is_empty());
        }
    }
}
