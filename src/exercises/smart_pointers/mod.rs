//! # Smart Pointers Exercises
//!
//! ## Learning Objectives
//! - Understand Box<T> for heap allocation
//! - Master reference counting with Rc<T> and Arc<T>
//! - Work with interior mutability using RefCell<T> and Cell<T>
//! - Implement custom smart pointers with Deref, DerefMut, and Drop
//! - Understand Cow<T> for clone-on-write semantics
//! - Handle weak references with Weak<T>
//! - Work with unsafe raw pointers
//! - Implement advanced memory patterns
//!
//! ## Topics Covered
//! - Box<T> basics and recursive types
//! - Rc<T> and Arc<T> for shared ownership
//! - RefCell<T> and Cell<T> for interior mutability
//! - Weak<T> for breaking reference cycles
//! - Cow<T> for efficient clone-on-write
//! - Custom smart pointers (Deref, DerefMut, Drop)
//! - Combining smart pointers (Rc+RefCell, Arc+Mutex)
//! - Unsafe raw pointers and custom allocators
//!
//! ## Difficulty Distribution (30 exercises)
//! - Easy: 8 exercises (01-08) - Box<T> basics
//! - Medium: 12 exercises (09-20) - Rc, Arc, RefCell, Cell
//! - Hard: 8 exercises (21-28) - Cow, custom smart pointers, patterns
//! - Expert: 2 exercises (29-30) - Unsafe, advanced patterns

pub mod exercise_01;
pub mod exercise_02;
pub mod exercise_03;
pub mod exercise_04;
pub mod exercise_05;
pub mod exercise_06;
pub mod exercise_07;
pub mod exercise_08;
pub mod exercise_09;
pub mod exercise_10;
pub mod exercise_11;
pub mod exercise_12;
pub mod exercise_13;
pub mod exercise_14;
pub mod exercise_15;
pub mod exercise_16;
pub mod exercise_17;
pub mod exercise_18;
pub mod exercise_19;
pub mod exercise_20;
pub mod exercise_21;
pub mod exercise_22;
pub mod exercise_23;
pub mod exercise_24;
pub mod exercise_25;
pub mod exercise_26;
pub mod exercise_27;
pub mod exercise_28;
pub mod exercise_29;
pub mod exercise_30;
