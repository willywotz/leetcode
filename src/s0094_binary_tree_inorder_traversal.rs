// problem: https://leetcode.com/problems/binary-tree-inorder-traversal/

pub struct Solution {}
use super::*;

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { return Vec::new(); }

        let mut result = Vec::new();
        let mut stack = VecDeque::new();
        let mut current = root;

        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push_back(node.clone());
                current = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop_back() {
                result.push(node.borrow().val);
                current = node.borrow().right.clone();
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test() {
        assert_eq!(vec![1,3,2], Solution::inorder_traversal(build_tree(&[Some(1),None,Some(2),Some(3)])));
        assert_eq!(vec![4,2,6,5,7,1,3,9,8], Solution::inorder_traversal(build_tree(&[Some(1),Some(2),Some(3),Some(4),Some(5),None,Some(8),None,None,Some(6),Some(7),Some(9)])));
        assert_eq!(Vec::<i32>::new(), Solution::inorder_traversal(build_tree(&[])));
        assert_eq!(vec![1], Solution::inorder_traversal(build_tree(&[Some(1)])));
    }
}
