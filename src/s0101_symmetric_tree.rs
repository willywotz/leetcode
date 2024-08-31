// problem: https://leetcode.com/problems/symmetric-tree/

pub struct Solution {}
use super::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() { return true; }

        let root = root.unwrap();
        let mut queue = VecDeque::new();
        queue.push_back(root.borrow().left.clone());
        queue.push_back(root.borrow().right.clone());

        while !queue.is_empty() {
            let left = queue.pop_front().unwrap();
            let right = queue.pop_front().unwrap();

            match (left.clone(), right.clone()) {
                (None, None) => continue,
                (Some(l), Some(r)) => {
                    if l.borrow().val != r.borrow().val {
                        return false;
                    }
                    queue.push_back(l.borrow().left.clone());
                    queue.push_back(r.borrow().right.clone());
                    queue.push_back(l.borrow().right.clone());
                    queue.push_back(r.borrow().left.clone());
                }
                _ => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_tree;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_symmetric(build_tree(&[Some(1),Some(2),Some(2),Some(3),Some(4),Some(4),Some(3)])));
        assert_eq!(false, Solution::is_symmetric(build_tree(&[Some(1),Some(2),Some(2),None,Some(3),None,Some(3)])));
    }
}
