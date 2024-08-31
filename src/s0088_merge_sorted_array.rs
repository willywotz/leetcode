// problem: https://leetcode.com/problems/merge-sorted-array/

pub struct Solution {}

impl Solution {
    pub fn merge(_nums1: &mut Vec<i32>, _m: i32, _nums2: &mut Vec<i32>, _n: i32) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums1 = vec![1,2,3,0,0,0];
        Solution::merge(&mut nums1, 3, &mut vec![2,5,6], 3);
        assert_eq!(&mut vec![1,2,2,3,5,6], &mut nums1);

        let mut nums1 = vec![1];
        Solution::merge(&mut nums1, 1, &mut vec![], 0);
        assert_eq!(&mut vec![1], &mut nums1);

        let mut nums1 = vec![0];
        Solution::merge(&mut nums1, 0, &mut vec![1], 1);
        assert_eq!(&mut vec![1], &mut nums1);
    }
}
