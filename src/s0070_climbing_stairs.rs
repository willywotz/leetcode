// problem: https://leetcode.com/problems/climbing-stairs/

pub struct Solution {}

impl Solution {
    pub fn climb_stairs(_n: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
