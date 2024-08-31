// problem: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

pub struct Solution {}
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(_nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test() {
        assert_eq!(build_tree(&[Some(0),Some(-3),Some(9),Some(-10),None,Some(5)]), Solution::sorted_array_to_bst(vec![-10,-3,0,5,9]));
        assert_eq!(build_tree(&[Some(3),Some(1)]), Solution::sorted_array_to_bst(vec![1,3]));
    }
}
