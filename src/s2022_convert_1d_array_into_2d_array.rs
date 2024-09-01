// problem: https://leetcode.com/problems/convert-1d-array-into-2d-array

pub struct Solution {}

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() != (m*n) as usize {
            return Vec::new();
        }

        let m = m as usize;
        let n = n as usize;

        let mut ret = vec![vec![0; n]; m];

        for i in 0..(m*n) {
            ret[i/n][i%n] = original[i];
        }

        ret
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
        assert_eq!(vec![vec![1],vec![1],vec![1],vec![1]], Solution::construct2_d_array(vec![1,1,1,1], 4, 1));
    }
}
