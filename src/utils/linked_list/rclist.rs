//! based on https://rust-unofficial.github.io/too-many-lists/third.html

// this is how to import our `Rc` only it does not have some minor functionality we need
// use crate::utils::smart_pointers::rc::Rc;
// use crate::utils::smart_pointers::refcell::RefCell;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: RefCell<T>,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }
    ///pushes an element onto the front of the list mutating it in place
    fn push_mut(&mut self, elem: T) {
        let head = self.head.clone();
        let new = Rc::new(Node {
            elem: RefCell::new(elem),
            next: head,
        });
        self.head.replace(new);
    }
    ///pushes an element onto the front of the list and returns a new list (including the original as the tail)
    fn push(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: RefCell::new(elem),
                next: self.head.clone(),
            })),
        }
    }

    fn tail(&self) -> Self {
        let tail = self.head.as_ref().and_then(|head| head.next.clone());

        List { head: tail }
    }
    fn head(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|head| head.elem.borrow())
    }
    fn iter<'a, I: From<&'a List<T>>>(&'a self) -> I {
        I::from(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(node) = curr {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                curr = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            node.elem.borrow()
        })
    }
}

impl<'a, T> From<&'a List<T>> for Iter<'a, T> {
    fn from(list: &'a List<T>) -> Iter<'a, T> {
        Iter {
            next: list.head.as_ref().map(|head| &**head),
        }
    }
}
pub struct IterMut<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> From<&'a List<T>> for IterMut<'a, T> {
    fn from(list: &'a List<T>) -> IterMut<'a, T> {
        IterMut {
            next: list.head.as_ref().map(|head| &**head),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = RefMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            node.elem.borrow_mut()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let list: List<i32> = List::new();

        assert!(list.head().is_none());

        let mut list = list.push(1).push(2);
        list.push_mut(3);

        assert_eq!(*list.head().unwrap(), 3);

        let list = list.tail();
        assert_eq!(*list.head().unwrap(), 2);

        let list = list.tail();
        assert_eq!(*list.head().unwrap(), 1);

        let list = list.tail();
        assert!(list.head().is_none());

        // Make sure empty tail works
        let list = list.tail();
        assert!(list.head().is_none());
    }

    #[test]
    fn iter() {
        let list = List::new().push(1).push(2).push(3);

        let mut iter: Iter<_> = list.iter();
        assert_eq!(iter.next().map(|n| *n), Some(3));
        assert_eq!(iter.next().map(|n| *n), Some(2));
        assert_eq!(iter.next().map(|n| *n), Some(1));
    }
}
