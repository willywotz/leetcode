// problem: https://leetcode.com/problems/merge-two-sorted-lists/

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(list1), None) => Some(list1),
            (None, Some(list2)) => Some(list2),
            (Some(mut list1), Some(mut list2)) => {
                if list1.val < list2.val {
                    list1.next = Solution::merge_two_lists(list1.next, Some(list2));
                    Some(list1)
                } else {
                    list2.next = Solution::merge_two_lists(Some(list1), list2.next);
                    Some(list2)
                }
            }
        }
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
        assert_eq!(create_list(vec![1,1,2,3,4,4]), Solution::merge_two_lists(create_list(vec![1,2,4]), create_list(vec![1,3,4])));
        assert_eq!(None, Solution::merge_two_lists(None, None));
        assert_eq!(create_list(vec![0]), Solution::merge_two_lists(None, create_list(vec![0])));
    }
}
