//! Exercise 23: Const Generics - Use const generic parameters
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Use const generics for array sizes
//! - Implement fixed-size generic containers
//! - Work with const generic parameters

/// A fixed-size stack using const generics.
pub struct Stack<T, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

impl<T, const N: usize> Stack<T, N> {
    /// Creates a new empty stack.
    pub fn new() -> Self
    where
        T: Copy,
     {
        todo!("Create a new empty stack.")
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, value: T) -> Result<(), &'static str>  {
        todo!("Pushes a value onto the stack.")
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self) -> Option<T>  {
        todo!("Pops a value from the stack.")
    }

    /// Returns the current length of the stack.
    pub fn len(&self) -> usize  {
        todo!("Return the current length of the stack.")
    }

    /// Checks if the stack is empty.
    pub fn is_empty(&self) -> bool  {
        todo!("Check if the stack is empty.")
    }

    /// Checks if the stack is full.
    pub fn is_full(&self) -> bool  {
        todo!("Check if the stack is full.")
    }

    /// Returns the capacity of the stack.
    pub fn capacity(&self) -> usize  {
        todo!("Return the capacity of the stack.")
    }
}

impl<T: Copy, const N: usize> Default for Stack<T, N> {
    fn default() -> Self  {
        todo!("Return the capacity of the stack.")
    }
}

/// A fixed-size ring buffer using const generics.
pub struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl<T, const N: usize> RingBuffer<T, N>
where
    T: Copy,
{
    /// Creates a new empty ring buffer.
    pub fn new() -> Self  {
        todo!("Create a new empty ring buffer.")
    }

    /// Writes a value to the buffer.
    pub fn write(&mut self, value: T) -> Result<(), &'static str>  {
        todo!("Writes a value to the buffer.")
    }

    /// Reads a value from the buffer.
    pub fn read(&mut self) -> Option<T>  {
        todo!("Reads a value from the buffer.")
    }

    /// Returns the number of elements in the buffer.
    pub fn len(&self) -> usize  {
        todo!("Return the number of elements in the buffer.")
    }

    /// Checks if the buffer is empty.
    pub fn is_empty(&self) -> bool  {
        todo!("Check if the buffer is empty.")
    }

    /// Checks if the buffer is full.
    pub fn is_full(&self) -> bool  {
        todo!("Check if the buffer is full.")
    }
}

impl<T: Copy, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self  {
        todo!("Check if the buffer is full.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_new() {
        let stack: Stack<i32, 5> = Stack::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.capacity(), 5);
    }

    #[test]
    fn test_stack_push() {
        let mut stack: Stack<i32, 5> = Stack::new();
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack: Stack<i32, 5> = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack: Stack<i32, 3> = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        assert!(stack.push(4).is_err());
    }

    #[test]
    fn test_stack_is_full() {
        let mut stack: Stack<i32, 2> = Stack::new();
        assert!(!stack.is_full());
        stack.push(1).unwrap();
        assert!(!stack.is_full());
        stack.push(2).unwrap();
        assert!(stack.is_full());
    }

    #[test]
    fn test_ring_buffer_new() {
        let buffer: RingBuffer<i32, 5> = RingBuffer::new();
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_ring_buffer_write_read() {
        let mut buffer: RingBuffer<i32, 5> = RingBuffer::new();
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), None);
    }

    #[test]
    fn test_ring_buffer_full() {
        let mut buffer: RingBuffer<i32, 3> = RingBuffer::new();
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();
        assert!(buffer.is_full());
        assert!(buffer.write(4).is_err());
    }

    #[test]
    fn test_ring_buffer_wraparound() {
        let mut buffer: RingBuffer<i32, 3> = RingBuffer::new();
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        assert_eq!(buffer.read(), Some(1));
        buffer.write(3).unwrap();
        buffer.write(4).unwrap();
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
    }
}
