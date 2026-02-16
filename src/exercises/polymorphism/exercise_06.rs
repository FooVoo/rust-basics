//! Exercise 06: Trait Bounds in Functions - Use traits as function parameters
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Use trait bounds in function signatures
//! - Pass different types that implement the same trait
//! - Understand static dispatch with impl Trait

pub trait Summarizable {
    fn summary(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub content: String,
}

impl Summarizable for Article {
    fn summary(&self) -> String {
        format!("Article: {} - {}", self.title, &self.content[..50.min(self.content.len())])
    }
}

pub struct Tweet {
    pub username: String,
    pub message: String,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.username, self.message)
    }
}

/// Function that takes any type implementing Summarizable
pub fn print_summary(item: &impl Summarizable) -> String {
    format!("Summary: {}", item.summary())
}

/// Alternative syntax using trait bounds
pub fn get_summary<T: Summarizable>(item: &T) -> String {
    item.summary()
}

/// Function that takes multiple items with the same trait
pub fn combine_summaries<T: Summarizable>(item1: &T, item2: &T) -> String {
    format!("{} | {}", item1.summary(), item2.summary())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_summary() {
        let article = Article {
            title: "Rust".to_string(),
            content: "Rust is a systems programming language that runs blazingly fast.".to_string(),
        };
        assert!(article.summary().contains("Article:"));
        assert!(article.summary().contains("Rust"));
    }

    #[test]
    fn test_tweet_summary() {
        let tweet = Tweet {
            username: "rustlang".to_string(),
            message: "Rust 1.70 is out!".to_string(),
        };
        assert!(tweet.summary().contains("@rustlang"));
        assert!(tweet.summary().contains("Rust 1.70"));
    }

    #[test]
    fn test_print_summary_article() {
        let article = Article {
            title: "Testing".to_string(),
            content: "This is a test article.".to_string(),
        };
        let result = print_summary(&article);
        assert!(result.contains("Summary:"));
    }

    #[test]
    fn test_print_summary_tweet() {
        let tweet = Tweet {
            username: "tester".to_string(),
            message: "Testing tweets".to_string(),
        };
        let result = print_summary(&tweet);
        assert!(result.contains("Summary:"));
    }

    #[test]
    fn test_get_summary_generic() {
        let article = Article {
            title: "Generic".to_string(),
            content: "Testing generic function.".to_string(),
        };
        let summary = get_summary(&article);
        assert!(summary.contains("Article:"));
    }

    #[test]
    fn test_combine_summaries() {
        let tweet1 = Tweet {
            username: "user1".to_string(),
            message: "First tweet".to_string(),
        };
        let tweet2 = Tweet {
            username: "user2".to_string(),
            message: "Second tweet".to_string(),
        };
        let combined = combine_summaries(&tweet1, &tweet2);
        assert!(combined.contains("@user1"));
        assert!(combined.contains("@user2"));
        assert!(combined.contains("|"));
    }
}
