//! 7.1 Merge two sorted lists
//! Consider two singly linked lists in which each node holds a number. Assume the lists are sorted,
//! i.e., numbers in the lists appear in ascending order within each list. The merge of the two lists is a
//! list consisting of the nodes of the two lists in which numbers appear in ascending order
// our custom linked list
// use crate::utils::linked_list::rclist::{Iter, List};
use std::collections::LinkedList;

fn merge_sorted_lists<T: Ord + Clone>(
    list1: &LinkedList<T>,
    list2: &LinkedList<T>,
) -> LinkedList<T> {
    let mut l1iter = list1.iter();
    let mut l2iter = list2.iter();
    let mut i1 = l1iter.next();
    let mut i2 = l2iter.next();
    let mut ret = LinkedList::new();
    loop {
        match (i1, i2) {
            (None, None) => break,
            (None, Some(n2)) => {
                i2 = l2iter.next();
                ret.push_back(n2.clone());
            }
            (Some(n1), None) => {
                i1 = l1iter.next();
                ret.push_back(n1.clone());
            }
            (Some(n1), Some(n2)) => {
                if n1 < n2 {
                    i1 = l1iter.next();
                    ret.push_back(n1.clone());
                } else {
                    i2 = l2iter.next();
                    ret.push_back(n2.clone());
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let mut list1 = LinkedList::new();
        list1.push_front(5);
        list1.push_front(3);
        list1.push_front(1);
        let mut list2 = LinkedList::new();
        list2.push_front(4);
        list2.push_front(3);
        list2.push_front(2);
        list2.push_front(0);
        let merged = merge_sorted_lists(&list1, &list2)
            .iter()
            .map(|v| *v)
            .collect::<Vec<_>>();
        assert_eq!(merged, vec![0, 1, 2, 3, 3, 4, 5]);
    }
}
