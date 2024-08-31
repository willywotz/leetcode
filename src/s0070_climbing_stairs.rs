// problem: https://leetcode.com/problems/climbing-stairs/

pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 || n == 2 {
            return n;
        }

        let mut s = vec![0; n as usize];
        s[0] = 1;
        s[1] = 2;

        for i in 2..n as usize {
            s[i] = s[i - 1] + s[i - 2];
        }

        s[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(5, Solution::climb_stairs(4));
    }
}
