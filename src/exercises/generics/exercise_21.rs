//! Exercise 21: Lifetimes with Generics - Combine lifetimes with generic parameters
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use lifetimes with generic types
//! - Understand lifetime bounds on generics
//! - Implement generic references

/// A generic wrapper that holds a reference with a lifetime.
pub struct Ref<'a, T> {
    value: &'a T,
}

impl<'a, T> Ref<'a, T> {
    /// Creates a new Ref.
    pub fn new(value: &'a T) -> Self {
        todo!("Implement new")
    }

    /// Gets the referenced value.
    pub fn get(&self) -> &T {
        todo!("Implement get")
    }
}

impl<'a, T: Clone> Ref<'a, T> {
    /// Clones the referenced value.
    pub fn cloned(&self) -> T {
        todo!("Implement cloned")
    }
}

/// A generic pair of references with potentially different lifetimes.
pub struct RefPair<'a, 'b, T, U> {
    first: &'a T,
    second: &'b U,
}

impl<'a, 'b, T, U> RefPair<'a, 'b, T, U> {
    /// Creates a new RefPair.
    pub fn new(first: &'a T, second: &'b U) -> Self {
        todo!("Implement new")
    }

    /// Gets the first reference.
    pub fn first(&self) -> &T {
        todo!("Implement first")
    }

    /// Gets the second reference.
    pub fn second(&self) -> &U {
        todo!("Implement second")
    }
}

/// A generic container that can hold either an owned value or a reference.
pub enum MaybeOwned<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<'a, T> MaybeOwned<'a, T> {
    /// Gets a reference to the value regardless of ownership.
    pub fn as_ref(&self) -> &T {
        todo!("Implement as_ref")
    }

    /// Checks if the value is owned.
    pub fn is_owned(&self) -> bool {
        todo!("Implement is_owned")
    }

    /// Checks if the value is borrowed.
    pub fn is_borrowed(&self) -> bool {
        todo!("Implement is_borrowed")
    }
}

impl<'a, T: Clone> MaybeOwned<'a, T> {
    /// Converts to an owned value.
    pub fn into_owned(self) -> T {
        todo!("Implement into_owned")
    }
}

impl<'a, T: ToOwned> MaybeOwned<'a, T>
where
    T::Owned: From<T>,
{
    /// Creates an owned variant from a value.
    pub fn from_value(value: T) -> MaybeOwned<'a, T::Owned> {
        todo!("Implement from_value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_new() {
        let value = 42;
        let r = Ref::new(&value);
        assert_eq!(*r.get(), 42);
    }

    #[test]
    fn test_ref_cloned() {
        let value = vec![1, 2, 3];
        let r = Ref::new(&value);
        let cloned = r.cloned();
        assert_eq!(cloned, vec![1, 2, 3]);
    }

    #[test]
    fn test_ref_string() {
        let s = String::from("hello");
        let r = Ref::new(&s);
        assert_eq!(r.get(), "hello");
    }

    #[test]
    fn test_ref_pair() {
        let a = 10;
        let b = "test";
        let pair = RefPair::new(&a, &b);
        assert_eq!(*pair.first(), 10);
        assert_eq!(*pair.second(), "test");
    }

    #[test]
    fn test_ref_pair_different_lifetimes() {
        let a = 42;
        {
            let b = 100;
            let _pair = RefPair::new(&a, &b);
        }
        assert_eq!(a, 42);
    }

    #[test]
    fn test_maybe_owned_owned() {
        let owned: MaybeOwned<i32> = MaybeOwned::Owned(42);
        assert!(owned.is_owned());
        assert!(!owned.is_borrowed());
        assert_eq!(*owned.as_ref(), 42);
    }

    #[test]
    fn test_maybe_owned_borrowed() {
        let value = 42;
        let borrowed: MaybeOwned<i32> = MaybeOwned::Borrowed(&value);
        assert!(borrowed.is_borrowed());
        assert!(!borrowed.is_owned());
        assert_eq!(*borrowed.as_ref(), 42);
    }

    #[test]
    fn test_maybe_owned_into_owned() {
        let value = 42;
        let borrowed: MaybeOwned<i32> = MaybeOwned::Borrowed(&value);
        let owned_value = borrowed.into_owned();
        assert_eq!(owned_value, 42);
    }

    #[test]
    fn test_maybe_owned_into_owned_already_owned() {
        let owned: MaybeOwned<String> = MaybeOwned::Owned("hello".to_string());
        let value = owned.into_owned();
        assert_eq!(value, "hello");
    }
}
