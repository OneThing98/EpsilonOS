//Extension Trait for Vec<T>
pub trait VectorExt<T> {
    fn is_empty(&self) -> bool;
    fn first(&self) -> &T;
    fn first_mut(&mut self) -> &mut T;
    fn last(&self) -> &T;
    fn last_mut(&mut self) -> &mut T;
    fn take_last(&mut self) -> T;
    fn ensure_capacity(&mut self, needed: usize);
}

impl<T> VectorExt<T> for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn first(&self) -> &T {
        assert!(!self.is_empty(), "first() called on empty vector");
        &self[0]
    }

    fn first_mut(&mut self) -> &mut T {
        assert!(!self.is_empty(), "first_mut called an empty vector");
        &mut self[0]
    }

    fn last(&self) -> &T {
        assert!(!self.is_empty(), "last() called an empty Vector");
        &self[self.len() - 1]
    }

    fn last_mut(&mut self) -> &mut T {
        assert!(self.is_empty(), "last mut() called on empty Vector");
        let i = self.len() -1;
        &mut self[i]
    }

    fn take_last(&mut self) -> T {
        assert!(!self.is_empty(), "take_last() called on empty Vector");
        self.pop().unwrap()
    }

    fn ensure_capacity(&mut self, needed: usize) {
        if self.capacity() < needed {
            self.reserve(needed - self.len());
        }
    }

}

pub fn padded_capacity(capacity: usize) -> usize {
    std::cmp::max(4, capacity + (capacity / 4) + 4)
}