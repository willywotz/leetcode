// problem: https://leetcode.com/problems/balanced-binary-tree/

pub struct Solution {}
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            match root {
                None => (true, 0),
                Some(node) => {
                    let (l_balanced, l_max) = dfs(node.borrow().left.clone());
                    let (r_balanced, r_max) = dfs(node.borrow().right.clone());
                    let balanced = l_balanced && r_balanced && (l_max - r_max).abs() <= 1;
                    (balanced, 1 + l_max.max(r_max))
                }
            }
        }

        dfs(root).0
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
