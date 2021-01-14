pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList { head: None }
    }
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let mut curr = self.head.as_mut();
        loop {
            match curr {
                Some(n) => match n.next {
                    Some(ref mut nxt) => {
                        curr = Some(nxt);
                    }
                    None => {
                        n.next = Some(new_node);
                        break;
                    }
                },
                None => {
                    self.head = Some(new_node);
                    break;
                }
            }
        }
    }
}

pub struct SinglyLinkedListIterator<'a, T> {
    curr: Option<&'a Node<T>>,
}

impl<'a, T> IntoIterator for &'a SinglyLinkedList<T> {
    type IntoIter = SinglyLinkedListIterator<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        SinglyLinkedListIterator {
            curr: if self.head.is_none() {
                None
            } else {
                Some(self.head.as_ref().unwrap().as_ref())
            },
        }
    }
}

pub struct SinglyLinkedListIteratorMut<'a, T> {
    curr: Option<&'a mut Node<T>>,
}

impl<'a, T> IntoIterator for &'a mut SinglyLinkedList<T> {
    type IntoIter = SinglyLinkedListIteratorMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        SinglyLinkedListIteratorMut {
            curr: if self.head.is_none() {
                None
            } else {
                Some(self.head.as_mut().unwrap().as_mut())
            },
        }
    }
}

impl<'a, T> Iterator for SinglyLinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            Some(n) => match n.next {
                Some(ref nxt) => {
                    let ret = self.curr;
                    self.curr = Some((*nxt).as_ref());

                    return Some(&ret.unwrap().value);
                }
                None => {
                    let ret = self.curr;
                    self.curr = None;
                    return Some(&ret.unwrap().value);
                }
            },
            None => return None,
        }
    }
}

impl<'a, T> Iterator for SinglyLinkedListIteratorMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr.take() {
            Some(n) => match n.next {
                Some(ref mut nxt) => {
                    let a = self.curr.replace(nxt.as_mut());
                    return Some(&mut a.unwrap().value);
                }
                None => {
                    return Some(&mut n.value);
                }
            },
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list_push() {
        let base = vec![310, 315, 275, 295, 260, 270, 290, 230, 255, 250];
        let mut ll = SinglyLinkedList::new();
        for i in &base {
            ll.push(*i);
        }
        assert_eq!(
            base.iter().collect::<Vec<_>>(),
            (&ll).into_iter().collect::<Vec<_>>()
        )
    }
    #[test]
    fn test_linked_list_iter_mut() {
        let base = vec![310, 315, 275, 295, 260, 270, 290, 230, 255, 250];
        let mut ll = SinglyLinkedList::new();
        for i in &base {
            ll.push(*i);
        }
        for i in &mut ll {
            *i = 5;
        }

        assert_eq!(
            (&ll).into_iter().collect::<Vec<_>>(),
            [5; 10].into_iter().collect::<Vec<_>>()
        )
    }
}
