pub mod s0001_two_sum;
pub mod s0009_palindrome_number;
pub mod s0013_roman_to_integer;
pub mod s0014_longest_common_prefix;
pub mod s0020_valid_parentheses;
pub mod s0021_merge_two_sorted_lists;
pub mod s0026_remove_duplicates_from_sorted_array;
pub mod s0027_remove_element;
pub mod s0028_find_the_index_of_the_first_occurrence_in_a_string;
pub mod s0035_search_insert_position;
pub mod s0058_length_of_last_word;
pub mod s0066_plus_one;
pub mod s0067_add_binary;
pub mod s0069_sqrtx;
pub mod s0070_climbing_stairs;
pub mod s0083_remove_duplicates_from_sorted_list;
pub mod s0088_merge_sorted_array;
pub mod s0094_binary_tree_inorder_traversal;
pub mod s0100_same_tree;
pub mod s0101_symmetric_tree;
pub mod s0104_maximum_depth_of_binary_tree;
pub mod s0108_convert_sorted_array_to_binary_search_tree;
pub mod s0110_balanced_binary_tree;

pub mod s2022_convert_1d_array_into_2d_array;
pub mod s2028_find_missing_observations;

use std::rc::Rc;
use std::cell::RefCell;

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

pub fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
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
