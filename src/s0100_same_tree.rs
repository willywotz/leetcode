// problem: https://leetcode.com/problems/same-tree/

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

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p_node = p.borrow();
                let q_node = q.borrow();

                p_node.val == q_node.val
                    && Solution::is_same_tree(p_node.left.clone(), q_node.left.clone())
                    && Solution::is_same_tree(p_node.right.clone(), q_node.right.clone())
            },
            (_, _) => false,
        }
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
        assert_eq!(true, Solution::is_same_tree(build_tree(&[Some(1),Some(2),Some(3)]), build_tree(&[Some(1),Some(2),Some(3)])));
        assert_eq!(false, Solution::is_same_tree(build_tree(&[Some(1),Some(2)]), build_tree(&[Some(1),None,Some(2)])));
        assert_eq!(false, Solution::is_same_tree(build_tree(&[Some(1),Some(2),Some(1)]), build_tree(&[Some(1),Some(1),Some(2)])));
    }
}
