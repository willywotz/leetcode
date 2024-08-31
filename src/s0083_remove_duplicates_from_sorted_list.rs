// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list/

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn delete_duplicates(_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for val in values {
            let node = ListNode::new(val);
            *current = Some(Box::new(node));
            current = &mut current.as_mut().unwrap().next;
        }

        head
    }

    #[test]
    fn test() {
        assert_eq!(create_list(vec![1,2]), Solution::delete_duplicates(create_list(vec![1,1,2])));
        assert_eq!(create_list(vec![1,2,3]), Solution::delete_duplicates(create_list(vec![1,1,2,3,3])));
    }
}
