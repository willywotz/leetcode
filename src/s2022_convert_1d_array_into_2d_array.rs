// problem: https://leetcode.com/problems/convert-1d-array-into-2d-array

pub struct Solution {}

impl Solution {
    pub fn construct2_d_array(_original: Vec<i32>, _m: i32, _n: i32) -> Vec<Vec<i32>> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![vec![1,2],vec![3,4]], Solution::construct2_d_array(vec![1,2,3,4], 2, 2));
        assert_eq!(vec![vec![1,2,3]], Solution::construct2_d_array(vec![1,2,3], 1, 3));
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::construct2_d_array(vec![1,2], 1, 1));
    }
}
