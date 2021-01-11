use std::rc::Rc;

pub struct SinglyLinkedList<T> {
    head: Option<Rc<Node<T>>>,
    length: usize,
}

pub struct SinglyLinkedListIterator<'a, T> {
    curr: Option<&'a Node<T>>,
}

pub struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            length: 0,
        }
    }
    pub fn push(&mut self, value: T) {
        let new = Some(Node::new(value));
        if self.head.is_none() {
            self.head = new;
            return;
        }
        let mut last = *(self.head.unwrap().next);
        while let Some(ref n) = last {
            match *n.next {
                Some(ref n) => last = &Some(*n),
                None => break,
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a SinglyLinkedList<T> {
    type Item = &'a Node<T>;
    type IntoIter = SinglyLinkedListIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        match self.head {
            None => {
                return SinglyLinkedListIterator::new(None);
            }
            Some(ref n) => {
                return SinglyLinkedListIterator::new(Some(n));
            }
        }
    }
}

impl<'a, T> Iterator for SinglyLinkedListIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            None => {
                return None;
            }
            Some(n) => {
                self.curr = (*n.next).as_ref();
                return Some(n);
            }
        }
    }
}

impl<'a, T> SinglyLinkedListIterator<'a, T> {
    fn new(curr: Option<&'a Node<T>>) -> Self {
        SinglyLinkedListIterator { curr }
    }
}
