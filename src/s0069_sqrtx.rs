// problem: https://leetcode.com/problems/sqrtx/

pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut size = x;
        let mut base = 1;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            if mid <= x / mid {
                base = mid;
            }
            size -= half;
        }
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
        assert_eq!(46339, Solution::my_sqrt(2147395599));
        assert_eq!(46340, Solution::my_sqrt(2147395600));
    }
}
