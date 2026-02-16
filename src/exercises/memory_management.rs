//! # Memory Management Exercises
//!
//! ## Learning Objectives
//! - Master ownership rules in Rust
//! - Understand borrowing and references
//! - Work with lifetimes effectively
//! - Manage memory safely without garbage collection
//! - Avoid common memory pitfalls
//!
//! ## Topics Covered
//! - Ownership transfer
//! - Borrowing (immutable and mutable)
//! - Lifetimes and lifetime elision
//! - Copy vs Clone traits
//! - Drop trait and RAII
//! - Memory safety guarantees
//!
//! ## Difficulty Distribution
//! - Easy: 5 exercises (01-05)
//! - Medium: 8 exercises (06-13)
//! - Hard: 5 exercises (14-18)
//! - Expert: 2 exercises (19-20)

// ============================================================================
// EASY EXERCISES (01-05): Basic ownership
// ============================================================================

/// Exercise 01: Ownership Transfer - Move semantics
/// Difficulty: Easy
pub fn take_ownership(s: String) -> String {
    s
}

/// Exercise 02: Clone Data - Create a copy
/// Difficulty: Easy
pub fn clone_string(s: &String) -> String {
    s.clone()
}

/// Exercise 03: Borrow Immutably - Read-only access
/// Difficulty: Easy
pub fn get_length(s: &String) -> usize {
    s.len()
}

/// Exercise 04: Borrow Mutably - Modify in place
/// Difficulty: Easy
pub fn append_exclamation(s: &mut String) {
    s.push('!');
}

/// Exercise 05: Multiple Immutable Borrows - Read from multiple references
/// Difficulty: Easy
pub fn concatenate_borrowed(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

// ============================================================================
// MEDIUM EXERCISES (06-13): References and lifetimes
// ============================================================================

/// Exercise 06: Return Reference - Return borrowed data
/// Difficulty: Medium
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    s
}

/// Exercise 07: Struct with References - Lifetime annotations
/// Difficulty: Medium
#[derive(Debug, PartialEq)]
pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str,
}

impl<'a> Book<'a> {
    pub fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }
    
    pub fn description(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }
}

/// Exercise 08: Longest String - Multiple lifetime parameters
/// Difficulty: Medium
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Exercise 09: Mutable Reference Rules - Single mutable borrow
/// Difficulty: Medium
pub fn modify_and_return(s: &mut String) -> &str {
    s.push_str(" modified");
    s.as_str()
}

/// Exercise 10: Ownership in Collections - Move into vector
/// Difficulty: Medium
pub fn create_string_vec() -> Vec<String> {
    vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ]
}

/// Exercise 11: Borrowing from Collections - Access without taking ownership
/// Difficulty: Medium
pub fn find_longest_string(strings: &[String]) -> Option<&String> {
    strings.iter().max_by_key(|s| s.len())
}

/// Exercise 12: Struct Ownership - Own vs borrow
/// Difficulty: Medium
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    pub fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

/// Exercise 13: Return Owned Data - Create and return
/// Difficulty: Medium
pub fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

// ============================================================================
// HARD EXERCISES (14-18): Complex lifetimes and patterns
// ============================================================================

/// Exercise 14: Struct with Multiple Lifetimes - Complex relationships
/// Difficulty: Hard
#[derive(Debug)]
pub struct Context<'a, 'b> {
    pub data: &'a str,
    pub metadata: &'b str,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(data: &'a str, metadata: &'b str) -> Self {
        Context { data, metadata }
    }
    
    pub fn get_data(&self) -> &'a str {
        self.data
    }
}

/// Exercise 15: Method with Lifetime - Return references from struct
/// Difficulty: Hard
pub struct TextBuffer {
    pub content: String,
}

impl TextBuffer {
    pub fn new(content: String) -> Self {
        TextBuffer { content }
    }
    
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.content.lines().nth(line_num)
    }
    
    pub fn first_line(&self) -> &str {
        self.content.lines().next().unwrap_or("")
    }
}

/// Exercise 16: Split Lifetime - Borrow parts of a struct
/// Difficulty: Hard
pub struct Pair {
    pub first: String,
    pub second: String,
}

impl Pair {
    pub fn new(first: String, second: String) -> Self {
        Pair { first, second }
    }
    
    pub fn get_first(&self) -> &str {
        &self.first
    }
    
    pub fn get_second(&self) -> &str {
        &self.second
    }
    
    pub fn get_both(&self) -> (&str, &str) {
        (&self.first, &self.second)
    }
}

/// Exercise 17: Generic Ownership - Work with generic types
/// Difficulty: Hard
pub struct Container<T> {
    pub value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
    
    pub fn get_ref(&self) -> &T {
        &self.value
    }
    
    pub fn take(self) -> T {
        self.value
    }
}

/// Exercise 18: Lifetime in Iterator - Return iterator with lifetime
/// Difficulty: Hard
pub struct Words<'a> {
    text: &'a str,
}

impl<'a> Words<'a> {
    pub fn new(text: &'a str) -> Self {
        Words { text }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.text.split_whitespace()
    }
}

// ============================================================================
// EXPERT EXERCISES (19-20): Advanced patterns
// ============================================================================

/// Exercise 19: Complex Lifetime Bounds - Generic with lifetime constraints
/// Difficulty: Expert
pub fn select_reference<'a, 'b, T>(
    x: &'a T,
    y: &'b T,
    use_first: bool,
) -> &'a T
where
    'b: 'a,
    T: PartialOrd,
{
    if use_first {
        x
    } else {
        // This works because 'b: 'a means 'b outlives 'a
        unsafe { &*(y as *const T) }
    }
}

// Safe version without unsafe
pub fn select_reference_safe<'a, T>(
    x: &'a T,
    y: &'a T,
    use_first: bool,
) -> &'a T {
    if use_first {
        x
    } else {
        y
    }
}

/// Exercise 20: Custom Drop Implementation - RAII pattern
/// Difficulty: Expert
pub struct Resource {
    pub name: String,
    pub is_dropped: bool,
}

impl Resource {
    pub fn new(name: String) -> Self {
        Resource {
            name,
            is_dropped: false,
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
    }
}

// Wrapper that tracks if drop was called (for testing)
pub struct TrackedResource {
    pub name: String,
}

impl TrackedResource {
    pub fn new(name: String) -> Self {
        TrackedResource { name }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Easy Tests
    #[test]
    fn test_take_ownership() {
        let s = String::from("hello");
        let result = take_ownership(s);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clone_string() {
        let s = String::from("hello");
        let cloned = clone_string(&s);
        assert_eq!(s, cloned);
        assert_eq!(s, "hello"); // Original still exists
    }

    #[test]
    fn test_get_length() {
        let s = String::from("hello");
        assert_eq!(get_length(&s), 5);
        assert_eq!(s, "hello"); // Original unchanged
    }

    #[test]
    fn test_append_exclamation() {
        let mut s = String::from("hello");
        append_exclamation(&mut s);
        assert_eq!(s, "hello!");
    }

    #[test]
    fn test_concatenate_borrowed() {
        let s1 = "hello";
        let s2 = " world";
        assert_eq!(concatenate_borrowed(s1, s2), "hello world");
    }

    // Medium Tests
    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_book() {
        let book = Book::new("1984", "George Orwell");
        assert_eq!(book.title, "1984");
        assert_eq!(book.author, "George Orwell");
        assert_eq!(book.description(), "'1984' by George Orwell");
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("short", "longer"), "longer");
        assert_eq!(longest("equal", "equal"), "equal");
    }

    #[test]
    fn test_modify_and_return() {
        let mut s = String::from("hello");
        let result = modify_and_return(&mut s);
        assert_eq!(result, "hello modified");
    }

    #[test]
    fn test_create_string_vec() {
        let vec = create_string_vec();
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], "hello");
    }

    #[test]
    fn test_find_longest_string() {
        let strings = vec![
            String::from("short"),
            String::from("medium"),
            String::from("longest"),
        ];
        assert_eq!(find_longest_string(&strings), Some(&String::from("longest")));
        
        let empty: Vec<String> = vec![];
        assert_eq!(find_longest_string(&empty), None);
    }

    #[test]
    fn test_person() {
        let person = Person::new(String::from("Alice"), 30);
        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 30);
        assert_eq!(person.greet(), "Hello, my name is Alice");
    }

    #[test]
    fn test_create_greeting() {
        assert_eq!(create_greeting("World"), "Hello, World!");
    }

    // Hard Tests
    #[test]
    fn test_context() {
        let data = "content";
        let metadata = "info";
        let ctx = Context::new(data, metadata);
        assert_eq!(ctx.get_data(), "content");
    }

    #[test]
    fn test_text_buffer() {
        let buffer = TextBuffer::new(String::from("line1\nline2\nline3"));
        assert_eq!(buffer.get_line(0), Some("line1"));
        assert_eq!(buffer.get_line(1), Some("line2"));
        assert_eq!(buffer.get_line(10), None);
        assert_eq!(buffer.first_line(), "line1");
    }

    #[test]
    fn test_pair() {
        let pair = Pair::new(String::from("first"), String::from("second"));
        assert_eq!(pair.get_first(), "first");
        assert_eq!(pair.get_second(), "second");
        assert_eq!(pair.get_both(), ("first", "second"));
    }

    #[test]
    fn test_container() {
        let container = Container::new(42);
        assert_eq!(*container.get_ref(), 42);
        assert_eq!(container.take(), 42);
    }

    #[test]
    fn test_words() {
        let words = Words::new("hello world rust");
        let collected: Vec<&str> = words.iter().collect();
        assert_eq!(collected, vec!["hello", "world", "rust"]);
    }

    // Expert Tests
    #[test]
    fn test_select_reference_safe() {
        let x = 10;
        let y = 20;
        assert_eq!(*select_reference_safe(&x, &y, true), 10);
        assert_eq!(*select_reference_safe(&x, &y, false), 20);
    }

    #[test]
    fn test_resource() {
        let resource = Resource::new(String::from("test"));
        assert_eq!(resource.name, "test");
        // Resource will be dropped at end of scope
    }

    #[test]
    fn test_tracked_resource() {
        let resource = TrackedResource::new(String::from("tracked"));
        assert_eq!(resource.name, "tracked");
    }
}
