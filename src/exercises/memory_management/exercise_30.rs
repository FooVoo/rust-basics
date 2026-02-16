//! Exercise 30: Zero-Cost Abstractions - Performance without overhead
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Implement zero-cost abstractions
//! - Use compile-time guarantees for safety
//! - Create efficient memory-safe APIs
//! - Understand static dispatch and monomorphization

use std::marker::PhantomData;

/// Type-state pattern for ensuring correct usage at compile time.
pub struct Initialized;
pub struct Uninitialized;

pub struct Builder<State> {
    data: Option<Vec<u8>>,
    _state: PhantomData<State>,
}

impl Builder<Uninitialized> {
    pub fn new() -> Self {
        Builder {
            data: None,
            _state: PhantomData,
        }
    }
    
    pub fn initialize(self, size: usize) -> Builder<Initialized> {
        Builder {
            data: Some(vec![0; size]),
            _state: PhantomData,
        }
    }
}

impl Builder<Initialized> {
    pub fn set(&mut self, index: usize, value: u8) {
        if let Some(ref mut data) = self.data {
            if index < data.len() {
                data[index] = value;
            }
        }
    }
    
    pub fn build(self) -> Vec<u8> {
        self.data.unwrap()
    }
}

/// Iterator wrapper with zero-cost abstraction.
pub struct ZipMap<I, J, F> {
    iter1: I,
    iter2: J,
    func: F,
}

impl<I, J, F, A, B, C> Iterator for ZipMap<I, J, F>
where
    I: Iterator<Item = A>,
    J: Iterator<Item = B>,
    F: FnMut(A, B) -> C,
{
    type Item = C;
    
    fn next(&mut self) -> Option<C> {
        match (self.iter1.next(), self.iter2.next()) {
            (Some(a), Some(b)) => Some((self.func)(a, b)),
            _ => None,
        }
    }
}

pub fn zip_map<I, J, F, A, B, C>(iter1: I, iter2: J, func: F) -> ZipMap<I, J, F>
where
    I: Iterator<Item = A>,
    J: Iterator<Item = B>,
    F: FnMut(A, B) -> C,
{
    ZipMap { iter1, iter2, func }
}

/// Compile-time bounds checking with const generics.
pub struct FixedBuffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedBuffer<N> {
    pub const fn new() -> Self {
        FixedBuffer {
            data: [0; N],
            len: 0,
        }
    }
    
    pub fn push(&mut self, value: u8) -> Result<(), &'static str> {
        if self.len >= N {
            return Err("Buffer full");
        }
        self.data[self.len] = value;
        self.len += 1;
        Ok(())
    }
    
    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.len {
            Some(self.data[index])
        } else {
            None
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn capacity(&self) -> usize {
        N
    }
}

/// Phantom type for units to prevent mixing incompatible types.
pub struct Meters;
pub struct Feet;

pub struct Distance<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Distance<Unit> {
    pub fn new(value: f64) -> Self {
        Distance {
            value,
            _unit: PhantomData,
        }
    }
    
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Distance<Meters> {
    pub fn to_feet(self) -> Distance<Feet> {
        Distance::new(self.value * 3.28084)
    }
}

impl Distance<Feet> {
    pub fn to_meters(self) -> Distance<Meters> {
        Distance::new(self.value / 3.28084)
    }
}

/// Add distances of the same unit (compile-time checked).
pub fn add_distances<Unit>(a: Distance<Unit>, b: Distance<Unit>) -> Distance<Unit> {
    Distance::new(a.value() + b.value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_typestate() {
        let builder = Builder::new();
        let mut builder = builder.initialize(10);
        builder.set(0, 42);
        builder.set(5, 100);
        
        let result = builder.build();
        assert_eq!(result[0], 42);
        assert_eq!(result[5], 100);
    }

    #[test]
    fn test_zip_map() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![10, 20, 30];
        
        let result: Vec<_> = zip_map(v1.into_iter(), v2.into_iter(), |a, b| a + b).collect();
        assert_eq!(result, vec![11, 22, 33]);
    }

    #[test]
    fn test_zip_map_multiply() {
        let v1 = vec![2, 3, 4];
        let v2 = vec![5, 6, 7];
        
        let result: Vec<_> = zip_map(v1.into_iter(), v2.into_iter(), |a, b| a * b).collect();
        assert_eq!(result, vec![10, 18, 28]);
    }

    #[test]
    fn test_fixed_buffer() {
        let mut buffer = FixedBuffer::<5>::new();
        
        assert_eq!(buffer.capacity(), 5);
        assert_eq!(buffer.len(), 0);
        
        assert!(buffer.push(1).is_ok());
        assert!(buffer.push(2).is_ok());
        
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.get(0), Some(1));
        assert_eq!(buffer.get(1), Some(2));
    }

    #[test]
    fn test_fixed_buffer_overflow() {
        let mut buffer = FixedBuffer::<2>::new();
        
        assert!(buffer.push(1).is_ok());
        assert!(buffer.push(2).is_ok());
        assert!(buffer.push(3).is_err());
    }

    #[test]
    fn test_distance_units() {
        let meters = Distance::<Meters>::new(100.0);
        assert_eq!(meters.value(), 100.0);
        
        let feet = meters.to_feet();
        assert!(feet.value() > 328.0 && feet.value() < 329.0);
    }

    #[test]
    fn test_add_distances() {
        let d1 = Distance::<Meters>::new(10.0);
        let d2 = Distance::<Meters>::new(20.0);
        
        let sum = add_distances(d1, d2);
        assert_eq!(sum.value(), 30.0);
    }

    #[test]
    fn test_distance_conversion_chain() {
        let meters = Distance::<Meters>::new(100.0);
        let feet = meters.to_feet();
        let meters_again = feet.to_meters();
        
        // Should be approximately 100.0 after round-trip conversion
        assert!((meters_again.value() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_const_buffer_sizes() {
        let small = FixedBuffer::<8>::new();
        let large = FixedBuffer::<256>::new();
        
        assert_eq!(small.capacity(), 8);
        assert_eq!(large.capacity(), 256);
    }
}
