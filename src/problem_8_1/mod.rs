//! 8.1 Implement a stack with a max api
//! Design a stack that includes a max operation, in addition to push and pop. The max method should
//! return the maximum value stored in the stack.
use std::rc::Rc;
#[derive(Debug)]
pub struct MaxStack<T: Ord> {
    stack: Vec<Rc<T>>,
    max_stack: Vec<Rc<T>>,
}
/// this implementation uses RC and two simultaneous stacks to cache the max value alongside each value
/// the max at any time is an RC to the pointer on the stack tracking the max `max_stack`
impl<T: Ord> MaxStack<T> {
    pub fn new() -> Self {
        MaxStack {
            stack: vec![],
            max_stack: vec![],
        }
    }
    pub fn push(&mut self, val: T) {
        let new = Rc::new(val);
        let max = self.max();

        if max.is_none() || (*new > **max.as_ref().unwrap()) {
            self.max_stack.push(new.clone());
        } else {
            self.max_stack.push(max.unwrap());
        }
        self.stack.push(new);
    }
    pub fn max(&self) -> Option<Rc<T>> {
        if self.max_stack.len() > 0 {
            return Some(self.max_stack[self.max_stack.len() - 1].clone());
        }
        None
    }
    pub fn pop(&mut self) -> Option<Rc<T>> {
        self.max_stack.pop();
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basics() {
        let mut stack = MaxStack::new();
        stack.push(4);

        assert_eq!(*stack.max().unwrap(), 4);
        stack.push(3);

        assert_eq!(*stack.max().unwrap(), 4);
        stack.push(5);
        assert_eq!(*stack.max().unwrap(), 5);
        assert_eq!(*stack.pop().unwrap(), 5);
        assert_eq!(*stack.max().unwrap(), 4);
    }
}
