//! `RefCell` is a type that provides interior mutability at runtime for types that are less efficiently `Copy`

use super::cell::Cell;
use std::cell::UnsafeCell;
use std::ops::Drop;
use std::ops::{Deref, DerefMut};

/// `BorrowStatus` represents the state of a `RefCell` at runtime
#[derive(Copy, Clone)]
enum BorrowStatus {
    Not,        // not borrowed
    Ref(usize), //borrowed n times: usize
    MutRef,     //borrowed once exclusively (mutable borrowed)
}

/// The Refcell itself
pub struct RefCell<T> {
    value: UnsafeCell<T>, //the value refcell is lending out; Unsafe cell affords us raw pointers to mutate the value
    _flag: Cell<BorrowStatus>, // the current status of the refcell so we know what kind of reference we can give out; this needs to
}

impl<T> RefCell<T> {
    // new refcell method
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            _flag: Cell::new(BorrowStatus::Not),
        }
    }
    /// `borrow` takes just `&self` and returns a `Ref` to the value inside if possible (depending on current status)
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        // need to check current status
        match self._flag.get() {
            // if not borrowed we can give one out
            BorrowStatus::Not => {
                self._flag.set(BorrowStatus::Ref(1));
                Some(Ref { refcell: self })
            }
            // if already immutable borrowed also safe to give out another
            BorrowStatus::Ref(n) => {
                self._flag.set(BorrowStatus::Ref(n + 1));
                Some(Ref { refcell: self })
            }
            // otherwise it is already exclusively borrowed and so we cannot give out anything
            BorrowStatus::MutRef => None,
        }
    }

    /// `borrow_mut` takes just `&self` and returns a `RefMut` to the value inside
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self._flag.get() {
            // We can only mutably borrow if there are no current references
            // otherwise we might be giving a mutable reference another value has reference to and the value could change underneath it
            BorrowStatus::Not => {
                self._flag.set(BorrowStatus::MutRef);
                Some(RefMut { refcell: self })
            }
            // if already borrowed or mutably borrowed then we cannot take any references
            _ => None,
        }
    }
}

/// the `Ref` object that will be used to return references with an api for semantic `Deref` and `Drop`
/// see `impl Drop for Ref` for more explanation
struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

/// `Deref` for `Ref` means getting a value to the actual `Refcell` value
/// i.e. *(&Ref) -> &Ref.recell.value
impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // we use the raw pointer given by `UnsafeCell` and cast it to a regular reference
        // SAFETY: This is safe because there can be no mutable references since we know we have a ref here our reftracking above dissallows there being a RefMut
        unsafe { &*self.refcell.value.get() }
    }
}

/// `Drop` for `Ref` is the real reason `Ref` exists
/// we need a way to track the borrow status when our `RefCell`'s borrows go out of scope so we can appropriately set the `BorrowStatus` flag
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell._flag.get() {
            BorrowStatus::Ref(n) => {
                if n > 1 {
                    self.refcell._flag.set(BorrowStatus::Ref(n - 1))
                } else {
                    self.refcell._flag.set(BorrowStatus::Not)
                }
            }

            _ => unreachable!(),
        }
    }
}

/// `Refmut` is analagous to 'Ref` but is returned by `Refcell`'s `borrow_mut`
/// it will allow dereferencing to a mutable reference to the `Refcell`'s internal value
struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

/// if we take a non mutable reference to `RefMut` to dereference i.e. `&RefMut` then we semantically yield a regular reference to the `Refcell` value
impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: no other Refs exist besides this one so it is safe to give out a real reference
        unsafe { &*self.refcell.value.get() }
    }
}
/// but unlike `Deref for RefMut` we return a mutable regular reference
impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: no other Refs exist besides this one so it is safe to give out a real mutable reference
        unsafe { &mut *self.refcell.value.get() }
    }
}

/// similar to `Drop for Ref` we need to change the `BorrowStatus` to unborrowed
impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        // this will have been the only reference since we are dropping a mutable reference
        self.refcell._flag.set(BorrowStatus::Not);
    }
}
