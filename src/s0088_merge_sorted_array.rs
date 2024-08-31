// problem: https://leetcode.com/problems/merge-sorted-array/

pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 { return; }
        if m == 0 { *nums1 = nums2.clone(); return; }

        let mut ret = vec![0; (m+n) as usize];
        let mut a: usize = 0;
        let mut b: usize = 0;

        for i in 0..(m+n) as usize {
            if a < m as usize && b == n as usize {
                ret[i] = nums1[a];
                a += 1;
            } else if b < n as usize && a == m as usize {
                ret[i] = nums2[b];
                b += 1;
            } else if nums1[a] < nums2[b] {
                ret[i] = nums1[a];
                a += 1;
            } else {
                ret[i] = nums2[b];
                b += 1;
            }
        }

        *nums1 = ret;
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

        let mut nums1 = vec![2,0];
        Solution::merge(&mut nums1, 1, &mut vec![1], 1);
        assert_eq!(&mut vec![1,2], &mut nums1);
    }
}
