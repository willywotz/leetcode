// problem: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

pub struct Solution {}
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;
        let mut root = TreeNode::new(nums[mid]);

        root.left = Solution::sorted_array_to_bst(nums[..mid].to_vec());
        root.right = Solution::sorted_array_to_bst(nums[mid + 1..].to_vec());

        Some(Rc::new(RefCell::new(root)))
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
