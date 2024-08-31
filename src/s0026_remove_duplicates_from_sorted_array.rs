// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array/

pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return  0;
        }

        let mut i = 0;

        for j in 0..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }

        nums.truncate(i + 1);
        (i + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = &mut vec![1,1,2];
        assert_eq!(2, Solution::remove_duplicates(nums));
        assert_eq!(&mut vec![1,2], nums);

        let nums = &mut vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(5, Solution::remove_duplicates(nums));
        assert_eq!(&mut vec![0,1,2,3,4], nums);
    }
}
