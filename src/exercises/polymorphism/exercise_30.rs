//! Exercise 30: Generic Associated Types (GATs) - Advanced type-level programming
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Understand Generic Associated Types (GATs)
//! - Use GATs for advanced abstractions
//! - Implement complex iterator patterns with GATs

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;
    
    fn next(&mut self) -> Option<Self::Item<'_>>;
}

pub struct WindowIterator<'data, T> {
    data: &'data [T],
    window_size: usize,
    position: usize,
}

impl<'data, T> WindowIterator<'data, T> {
    pub fn new(data: &'data [T], window_size: usize) -> Self  {
        todo!("Implement new")
    }
}

impl<'data, T> LendingIterator for WindowIterator<'data, T> {
    type Item<'a> = &'a [T]
    where
        Self: 'a;
    
    fn next(&mut self) -> Option<Self::Item<'_>>  {
        todo!("Implement next")
    }
}

pub trait Container {
    type Item<'a>
    where
        Self: 'a;
    
    fn get(&self, index: usize) -> Option<Self::Item<'_>>;
    fn len(&self) -> usize;
}

pub struct StringContainer {
    data: Vec<String>,
}

impl StringContainer {
    pub fn new() -> Self  {
        todo!("Implement new")
    }
    
    pub fn push(&mut self, s: String)  {
        todo!("Implement push")
    }
}

impl Container for StringContainer {
    type Item<'a> = &'a str
    where
        Self: 'a;
    
    fn get(&self, index: usize) -> Option<Self::Item<'_>>  {
        todo!("Implement get")
    }
    
    fn len(&self) -> usize  {
        todo!("Implement len")
    }
}

pub trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;
    
    fn advance(&mut self);
    fn get(&self) -> Option<Self::Item<'_>>;
    
    fn count(mut self) -> usize
    where
        Self: Sized,
     {
        todo!("Implement count")
    }
}

pub struct ChunkIterator<'data, T> {
    data: &'data [T],
    chunk_size: usize,
    position: usize,
}

impl<'data, T> ChunkIterator<'data, T> {
    pub fn new(data: &'data [T], chunk_size: usize) -> Self  {
        todo!("Implement new")
    }
}

impl<'data, T> StreamingIterator for ChunkIterator<'data, T> {
    type Item<'a> = &'a [T]
    where
        Self: 'a;
    
    fn advance(&mut self)  {
        todo!("Implement advance")
    }
    
    fn get(&self) -> Option<Self::Item<'_>>  {
        todo!("Implement get")
    }
}

pub trait PointerFamily {
    type Pointer<T>: std::ops::Deref<Target = T>;
    
    fn new<T>(value: T) -> Self::Pointer<T>;
}

pub struct BoxFamily;

impl PointerFamily for BoxFamily {
    type Pointer<T> = Box<T>;
    
    fn new<T>(value: T) -> Self::Pointer<T>  {
        todo!("Implement new")
    }
}

pub struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T> = std::rc::Rc<T>;
    
    fn new<T>(value: T) -> Self::Pointer<T>  {
        todo!("Implement new")
    }
}

pub trait Mapper {
    type Input<'a>
    where
        Self: 'a;
    type Output<'a>
    where
        Self: 'a;
    
    fn map<'a>(&'a self, input: Self::Input<'a>) -> Self::Output<'a>;
}

pub struct StringMapper;

impl Mapper for StringMapper {
    type Input<'a> = &'a str
    where
        Self: 'a;
    type Output<'a> = String
    where
        Self: 'a;
    
    fn map<'a>(&'a self, input: Self::Input<'a>) -> Self::Output<'a>  {
        todo!("Implement map")
    }
}

pub trait Collection {
    type Item<'a>
    where
        Self: 'a;
    type Iter<'a>: LendingIterator<Item<'a> = Self::Item<'a>>
    where
        Self: 'a;
    
    fn iter(&self) -> Self::Iter<'_>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_iterator() {
        let data = vec![1, 2, 3, 4, 5];
        let mut iter = WindowIterator::new(&data, 3);
        
        assert_eq!(iter.next(), Some(&[1, 2, 3][..]));
        assert_eq!(iter.next(), Some(&[2, 3, 4][..]));
        assert_eq!(iter.next(), Some(&[3, 4, 5][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_window_iterator_size_2() {
        let data = vec![10, 20, 30, 40];
        let mut iter = WindowIterator::new(&data, 2);
        
        assert_eq!(iter.next(), Some(&[10, 20][..]));
        assert_eq!(iter.next(), Some(&[20, 30][..]));
        assert_eq!(iter.next(), Some(&[30, 40][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_string_container() {
        let mut container = StringContainer::new();
        container.push("hello".to_string());
        container.push("world".to_string());
        
        assert_eq!(container.get(0), Some("hello"));
        assert_eq!(container.get(1), Some("world"));
        assert_eq!(container.get(2), None);
        assert_eq!(container.len(), 2);
    }

    #[test]
    fn test_chunk_iterator() {
        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let mut iter = ChunkIterator::new(&data, 3);
        
        assert_eq!(iter.get(), Some(&[1, 2, 3][..]));
        iter.advance();
        assert_eq!(iter.get(), Some(&[4, 5, 6][..]));
        iter.advance();
        assert_eq!(iter.get(), Some(&[7][..]));
        iter.advance();
        assert_eq!(iter.get(), None);
    }

    #[test]
    fn test_chunk_iterator_count() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let iter = ChunkIterator::new(&data, 3);
        assert_eq!(iter.count(), 3); // [1,2,3], [4,5,6], [7,8,9]
    }

    #[test]
    fn test_box_family() {
        let ptr = BoxFamily::new(42);
        assert_eq!(*ptr, 42);
    }

    #[test]
    fn test_rc_family() {
        let ptr = RcFamily::new("hello");
        assert_eq!(*ptr, "hello");
    }

    #[test]
    fn test_string_mapper() {
        let mapper = StringMapper;
        assert_eq!(mapper.map("hello"), "HELLO");
        assert_eq!(mapper.map("world"), "WORLD");
    }

    #[test]
    fn test_gat_lifetime_flexibility() {
        let data = vec![1, 2, 3, 4];
        let mut iter = WindowIterator::new(&data, 2);
        
        // Each call to next() can have different lifetimes
        {
            let window1 = iter.next();
            assert!(window1.is_some());
        }
        
        {
            let window2 = iter.next();
            assert!(window2.is_some());
        }
    }

    #[test]
    fn test_container_multiple_accesses() {
        let mut container = StringContainer::new();
        container.push("first".to_string());
        container.push("second".to_string());
        container.push("third".to_string());
        
        // Multiple borrows with GATs
        let item1 = container.get(0);
        let item2 = container.get(1);
        
        assert_eq!(item1, Some("first"));
        assert_eq!(item2, Some("second"));
    }

    #[test]
    fn test_streaming_iterator_pattern() {
        let data = vec![10, 20, 30, 40, 50];
        let mut iter = ChunkIterator::new(&data, 2);
        
        let mut chunks = Vec::new();
        while let Some(chunk) = iter.get() {
            chunks.push(chunk.to_vec());
            iter.advance();
        }
        
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], vec![10, 20]);
        assert_eq!(chunks[1], vec![30, 40]);
        assert_eq!(chunks[2], vec![50]);
    }

    #[test]
    fn test_pointer_family_abstraction() {
        fn use_pointer<F: PointerFamily>(value: i32) -> i32 {
            let ptr = F::new(value);
            *ptr * 2
        }
        
        assert_eq!(use_pointer::<BoxFamily>(21), 42);
        assert_eq!(use_pointer::<RcFamily>(21), 42);
    }

    #[test]
    fn test_mapper_with_different_inputs() {
        let mapper = StringMapper;
        
        let inputs = vec!["hello", "world", "rust"];
        let outputs: Vec<_> = inputs.iter().map(|s| mapper.map(s)).collect();
        
        assert_eq!(outputs, vec!["HELLO", "WORLD", "RUST"]);
    }

    #[test]
    fn test_lending_iterator_borrowing() {
        let data = vec!["a", "b", "c", "d"];
        let mut iter = WindowIterator::new(&data, 2);
        
        // Each window borrows from the iterator's lifetime
        if let Some(window) = iter.next() {
            assert_eq!(window.len(), 2);
        }
        
        if let Some(window) = iter.next() {
            assert_eq!(window.len(), 2);
        }
    }

    #[test]
    fn test_gat_type_flexibility() {
        // GATs allow the associated type to be generic over lifetimes
        let container = StringContainer::new();
        
        // Type Item<'_> can have different lifetimes for each call
        let _item1 = container.get(0); // Item<'a1>
        let _item2 = container.get(1); // Item<'a2>
    }

    #[test]
    fn test_complex_gat_usage() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let mut windows = WindowIterator::new(&data, 3);
        
        let mut sums = Vec::new();
        while let Some(window) = windows.next() {
            let sum: i32 = window.iter().sum();
            sums.push(sum);
        }
        
        assert_eq!(sums, vec![6, 9, 12, 15]); // [1+2+3], [2+3+4], [3+4+5], [4+5+6]
    }
}
