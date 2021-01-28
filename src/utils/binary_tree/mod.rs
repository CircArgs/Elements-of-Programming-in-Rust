type Node<T> = Option<Box<TreeNode<T>>>;
#[derive(Debug)]
pub struct TreeNode<T> {
    left: Node<T>,
    right: Node<T>,
    data: T,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            left: None,
            right: None,
            data: value,
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        if *&value < self.data {
            if self.left.is_none() {
                self.left = Some(Box::new(TreeNode::new(value)));
            } else {
                self.left.as_mut().unwrap().insert(value);
            }
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(TreeNode::new(value)));
            } else {
                self.right.as_mut().unwrap().insert(value);
            }
        }
    }
}
#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Node<T>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }
    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
        } else {
            self.root.as_mut().unwrap().insert(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut tree = BinaryTree::new();
        tree.insert(1);
        tree.insert(0);
        tree.insert(5);
        tree.insert(4);
        tree.insert(6);
        tree.insert(-2);
        assert_eq!(tree.root.as_ref().unwrap().data, 1);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().data, 0);
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            -2
        );
        assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().data, 5);
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .data,
            6
        );
        assert_eq!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .data,
            4
        );
    }
}
