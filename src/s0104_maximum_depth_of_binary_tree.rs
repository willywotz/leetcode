// problem: https://leetcode.com/problems/maximum-depth-of-binary-tree/

pub struct Solution {}
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left_depth = Solution::max_depth(node.left.clone());
            let right_depth = Solution::max_depth(node.right.clone());
            1 + left_depth.max(right_depth)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test() {
        assert_eq!(3, Solution::max_depth(build_tree(&[Some(3),Some(9),Some(20),None,None,Some(15),Some(7)])));
        assert_eq!(2, Solution::max_depth(build_tree(&[Some(1),None,Some(2)])));
    }
}
