// problem: https://leetcode.com/problems/binary-tree-inorder-traversal/

pub struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

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

    fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while i < values.len() {
            let node = queue.pop_front().unwrap();

            if let Some(&Some(val)) = values.get(i) {
                let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(Rc::clone(&left_child));
                queue.push_back(left_child);
            }
            i += 1;

            if i < values.len() {
                if let Some(&Some(val)) = values.get(i) {
                    let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right_child));
                    queue.push_back(right_child);
                }
                i += 1;
            }
        }

        Some(root)
    }

    #[test]
    fn test() {
        assert_eq!(vec![1,3,2], Solution::inorder_traversal(build_tree(&[Some(1),None,Some(2),Some(3)])));
        assert_eq!(vec![4,2,6,5,7,1,3,9,8], Solution::inorder_traversal(build_tree(&[Some(1),Some(2),Some(3),Some(4),Some(5),None,Some(8),None,None,Some(6),Some(7),Some(9)])));
        assert_eq!(Vec::<i32>::new(), Solution::inorder_traversal(build_tree(&[])));
        assert_eq!(vec![1], Solution::inorder_traversal(build_tree(&[Some(1)])));
    }
}
