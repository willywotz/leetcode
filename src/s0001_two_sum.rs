// problem: https://leetcode.com/problems/two-sum/

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                Some(&j) => return vec![j, i as i32],
                None => map.insert(nums[i], i as i32)
            };
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
        assert_eq!(vec![0, 3], Solution::two_sum(vec![0, 4, 3, 0], 0));
    }
}
