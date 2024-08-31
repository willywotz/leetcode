// problem: https://leetcode.com/problems/same-tree/

pub struct Solution {}
use super::*;

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
    use crate::build_tree;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_same_tree(build_tree(&[Some(1),Some(2),Some(3)]), build_tree(&[Some(1),Some(2),Some(3)])));
        assert_eq!(false, Solution::is_same_tree(build_tree(&[Some(1),Some(2)]), build_tree(&[Some(1),None,Some(2)])));
        assert_eq!(false, Solution::is_same_tree(build_tree(&[Some(1),Some(2),Some(1)]), build_tree(&[Some(1),Some(1),Some(2)])));
    }
}
