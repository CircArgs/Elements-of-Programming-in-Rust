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
    pub fn inorder_iter<'a>(&'a self) -> InorderTraversal<'a, T> {
        InorderTraversal {
            parent: &self.root,
            rel: NodeRelation::Left,
        }
    }
}

#[derive(Clone, Copy)]
enum NodeRelation {
    Parent,
    Left,
    Right,
}
pub struct InorderTraversal<'a, T> {
    parent: &'a Node<T>,
    rel: NodeRelation,
}

impl<'a, T> Iterator for InorderTraversal<'a, T> {
    type Item = &'a Box<TreeNode<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.parent.is_none() {
            return None;
        } else {
            match self.rel {
                NodeRelation::Left => {
                    let temp = self.parent.as_ref().unwrap().left.as_ref();
                    self.rel = NodeRelation::Parent;
                    if temp.is_none() {
                        return self.next();
                    } else {
                        return temp;
                    }
                }
                NodeRelation::Parent => {
                    let temp = self.parent.as_ref();
                    self.rel = NodeRelation::Right;
                    return temp;
                }
                NodeRelation::Right => {
                    let temp = self.parent.as_ref().unwrap().right.as_ref();
                    self.rel = NodeRelation::Left;
                    self.parent = &self.parent.as_ref().unwrap().left;
                    if temp.is_none() {
                        return self.next();
                    } else {
                        return temp;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tree1() -> BinaryTree<i32> {
        let mut tree = BinaryTree::new();
        tree.insert(1);
        tree.insert(0);
        tree.insert(5);
        tree.insert(4);
        tree.insert(6);
        tree.insert(-2);
        tree
    }

    #[test]
    fn test_insert() {
        let tree = tree1();
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
    #[test]
    fn test_inorder_iter() {
        let tree = tree1();
        let comp = tree.inorder_iter().map(|e| e.data).collect::<Vec<_>>();
        assert_eq!(comp, vec![0, 1, 5, -2, 4, 6]);
    }
}
