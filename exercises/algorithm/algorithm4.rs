use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    /// 递归插入值到当前节点的子树中
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 新值小于当前节点值，处理左子树
            Ordering::Less => {
                if let Some(ref mut left_node) = self.left {
                    // 左子树存在，递归插入
                    left_node.insert(value);
                } else {
                    // 左子树为空，创建新节点作为左子树
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            // 新值大于当前节点值，处理右子树
            Ordering::Greater => {
                if let Some(ref mut right_node) = self.right {
                    // 右子树存在，递归插入
                    right_node.insert(value);
                } else {
                    // 右子树为空，创建新节点作为右子树
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            // 新值等于当前节点值，忽略重复值
            Ordering::Equal => (),
        }
    }

    /// 递归查找值在当前节点的子树中是否存在
    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            // 目标值等于当前节点值，找到
            Ordering::Equal => true,
            // 目标值小于当前节点值，查找左子树
            Ordering::Less => self.left.as_ref().map_or(false, |left_node| left_node.search(value)),
            // 目标值大于当前节点值，查找右子树
            Ordering::Greater => self.right.as_ref().map_or(false, |right_node| right_node.search(value)),
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    /// 插入值到二叉搜索树中
    fn insert(&mut self, value: T) {
        if let Some(ref mut root_node) = self.root {
            // 根节点存在，调用节点的insert方法
            root_node.insert(value);
        } else {
            // 根节点为空，创建新节点作为根
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    /// 查找值是否存在于二叉搜索树中
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root_node| root_node.search(&value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 初始树为空，查找1应该返回false
        assert_eq!(bst.search(1), false);

        // 插入一系列值
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 验证存在的数值
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 验证不存在的数值
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复值1
        bst.insert(1);
        bst.insert(1);

        // 验证1存在
        assert_eq!(bst.search(1), true);

        // 验证根节点没有左右子树（重复值未被插入）
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
