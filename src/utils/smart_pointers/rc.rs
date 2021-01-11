//! `Rc` is a type that implements reference counting at runtime
//! it's purpose is, unlike `Cell` and `RefCell`, not to supply interior mutability
//! but to facilitate sharing a value in multiple places when lifetimes are not certain
//! an `Rc<T>` does not give out means by which to mutate the `T` only ones to get reference to it

use std::marker::PhantomData;
use std::ops::{Deref, Drop};

/// `RcInner` is what an `Rc` actually stores a reference to.
/// it is necessary in a similar but not the same way that `Ref` and `RefMut` are necessary to `RefCell`
/// when an `Rc` is cloned we need a way to continue tracking the refcount and so we do this by storing a raw pointer to `RcInner`
/// and when the `Rc` is cloned the _raw pointer_ to the `RcInner` is cloned too
struct RcInner<T> {
    value: T,
    refcount: usize,
}

/// the actual `Rc` object
pub struct Rc<T> {
    // the inner value does all the lifting and as mentioned in the docs for `RcInner` it is pointed to by a raw pointer to maintain tracking of the reference count across clones
    inner: *mut RcInner<T>,
    // phantom data is used to ensure that the "Drop Check"  as conducted by rust ( see https://doc.rust-lang.org/nomicon/dropck.html ) when an `Rc` is dropped is done properly accounting
    // for the fact that `Rc` may also be initiating the drop of a `T` which it does (see `impl Drop for Rc` below) when the refcount drops to 0
    _marker: PhantomData<T>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        // get the raw pointer to the `RcInner`
        let inner = Box::into_raw(Box::new(RcInner { value, refcount: 1 }));
        Rc {
            inner,
            _marker: PhantomData,
        }
    }
}

/// `Rc` implements clone as the method by which we facilitate sharing the value
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        // a cloning increases the reference count
        unsafe {
            (*self.inner).refcount += 1;
        }
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

/// we want to be able to dereference `Rc<T>` to get reference to the `T` inside
impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.inner).value }
    }
}

/// where the reference decrementing happens
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        // check the value of the refcount
        let refcount = unsafe { (*self.inner).refcount };
        if refcount > 1 {
            // decrement when there are multiple references
            unsafe { (*self.inner).refcount -= 1 };
        } else {
            // but if there is just one then the Rc needs to be dropped
            drop(unsafe { Box::from_raw(self.inner) });
        }
    }
}
