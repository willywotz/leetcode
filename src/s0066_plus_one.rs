// problem: https://leetcode.com/problems/plus-one/

pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry: i32 = 1;

        for i in (0..digits.len()).rev() {
            if carry != 0 { digits[i] += carry; carry = 0; }
            if digits[i] >= 10 { digits[i] %= 10; carry += 1; }
        }

        if carry != 0 {
            digits.insert(0, carry);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![1,2,4], Solution::plus_one(vec![1,2,3]));
        assert_eq!(vec![4,3,2,2], Solution::plus_one(vec![4,3,2,1]));
        assert_eq!(vec![1,0], Solution::plus_one(vec![9]));
        assert_eq!(vec![1,0,0], Solution::plus_one(vec![9,9]));
        assert_eq!(vec![9,0,0,0], Solution::plus_one(vec![8,9,9,9]));
        assert_eq!(vec![5,6,2,0,0,4,6,2,5,0], Solution::plus_one(vec![5,6,2,0,0,4,6,2,4,9]));
    }
}
