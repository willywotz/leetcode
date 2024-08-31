// problem: https://leetcode.com/problems/remove-element/

pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = &mut vec![3,2,2,3];
        assert_eq!(2, Solution::remove_element(nums, 3));
        assert_eq!(&vec![2,2], nums);

        let nums = &mut vec![0,1,2,2,3,0,4,2];
        assert_eq!(5, Solution::remove_element(nums, 2));
        assert_eq!(&vec![0,1,3,0,4], nums);
    }
}
