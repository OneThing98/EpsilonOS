pub struct OwnPtr<T> {
    inner: Option<Box<T>>,
}

impl<T> OwnPtr<T> {

    pub fn new() -> Self {
        OwnPtr { inner: None }
    }

    pub fn from_value(value: T) -> Self {
        OwnPtr{
            inner: Some(Box::new(value)),
        }
    }

    pub fn make(value: T) -> Self {
        Self::from_value(value)
    }

    pub fn is_valid(&self) -> bool {
        self.inner.is_some()
    }

    pub fn ptr(&self) -> Option<&T> {
        self.inner.as_deref()
    }

    pub fn ptr_mut(&mut self) -> Option<&mut T> {
        self.inner.as_deref_mut()
    }

    pub fn leak_ptr(&mut self) -> Option<Box<T>> {
        self.inner.take()
    }

    pub fn clear(&mut self) {
        self.inner = None;
    }
}

impl<T> Default for OwnPtr<T> {
    fn default() -> Self {
        OwnPtr::new()
    }
}

pub fn make<T>(value: T) -> OwnPtr<T> {
    OwnPtr::make(value)
}