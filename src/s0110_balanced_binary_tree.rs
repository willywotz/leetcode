// problem: https://leetcode.com/problems/balanced-binary-tree/

pub struct Solution {}
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_balanced(build_tree(&[Some(3),Some(9),Some(20),None,None,Some(15),Some(7)])));
        assert_eq!(false, Solution::is_balanced(build_tree(&[Some(1),Some(2),Some(2),Some(3),Some(3),None,None,Some(4),Some(4)])));
        assert_eq!(true, Solution::is_balanced(None));
    }
}
