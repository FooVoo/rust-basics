//! Exercise 21: Multiple Lifetime Parameters - Complex lifetime relationships
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Work with multiple lifetime parameters
//! - Understand lifetime relationships
//! - Handle complex borrowing scenarios

/// Choose the string with the longer lifetime.
pub fn choose_str<'a, 'b>(x: &'a str, y: &'b str, use_first: bool) -> &'a str
where
    'b: 'a,
 {
    todo!("Choose the string with the longer lifetime.")
}

pub struct Context<'s, 't> {
    source: &'s str,
    target: &'t str,
}

impl<'s, 't> Context<'s, 't> {
    pub fn new(source: &'s str, target: &'t str) -> Self {
        todo!("Implement new")
    }
    
    pub fn source(&self) -> &'s str {
        todo!("Implement source")
    }
    
    pub fn target(&self) -> &'t str {
        todo!("Implement target")
    }
    
    pub fn combine(&self) -> String {
        todo!("Implement combine")
    }
}

/// Create a context and get combined string.
pub fn process_context<'a, 'b>(source: &'a str, target: &'b str) -> String {
    todo!("Implement process_context")
}

pub struct RefPair<'a, 'b, T> {
    first: &'a T,
    second: &'b T,
}

impl<'a, 'b, T> RefPair<'a, 'b, T> {
    pub fn new(first: &'a T, second: &'b T) -> Self {
        todo!("Implement new")
    }
    
    pub fn first(&self) -> &'a T {
        todo!("Implement first")
    }
    
    pub fn second(&self) -> &'b T {
        todo!("Implement second")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_str() {
        let s1 = String::from("first");
        let s2 = String::from("second");
        
        let result = choose_str(&s1, &s2, true);
        assert_eq!(result, "first");
        
        let result2 = choose_str(&s1, &s2, false);
        assert_eq!(result2, "second");
    }

    #[test]
    fn test_context() {
        let source = "input";
        let target = "output";
        let ctx = Context::new(source, target);
        
        assert_eq!(ctx.source(), "input");
        assert_eq!(ctx.target(), "output");
        assert_eq!(ctx.combine(), "input -> output");
    }

    #[test]
    fn test_process_context() {
        let result = process_context("hello", "world");
        assert_eq!(result, "hello -> world");
    }

    #[test]
    fn test_ref_pair() {
        let x = 42;
        let y = 100;
        let pair = RefPair::new(&x, &y);
        
        assert_eq!(*pair.first(), 42);
        assert_eq!(*pair.second(), 100);
    }

    #[test]
    fn test_different_lifetimes() {
        let long_lived = String::from("long");
        let pair;
        {
            let short_lived = String::from("short");
            let ctx = Context::new(&long_lived, &short_lived);
            pair = (ctx.source().to_string(), ctx.target().to_string());
        }
        assert_eq!(pair.0, "long");
        assert_eq!(pair.1, "short");
    }
}
