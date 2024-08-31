// problem: https://leetcode.com/problems/search-insert-position/

pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::search_insert(vec![1,3,5,6], 5));
        assert_eq!(1, Solution::search_insert(vec![1,3,5,6], 2));
        assert_eq!(4, Solution::search_insert(vec![1,3,5,6], 7));
    }
}
