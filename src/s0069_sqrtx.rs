// problem: https://leetcode.com/problems/sqrtx/

pub struct Solution {}

impl Solution {
    pub fn my_sqrt(_x: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
    }
}
