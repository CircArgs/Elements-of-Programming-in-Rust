use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }
    pub fn set(&self, value: T) {
        //SAFETY: we know there are no other threads editing because Cell !Sync via UnsafeCell
        //SAFETY: no references are even given out via `get` only cloned vlaues
        unsafe {
            *self.value.get() = value;
        }
    }
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: only one thread can have a reference at once since !Sync so nothing could be mutating this value as we copy and return it
        unsafe { *self.value.get() } 
    }
}
