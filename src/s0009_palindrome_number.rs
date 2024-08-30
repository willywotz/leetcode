pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x == 0 {
            return true
        } else if x < 0 || x % 10 == 0 {
            return false
        }

        let mut tmp = x;
        let mut mem: i32 = 0;

        while tmp != 0 {
            mem = mem * 10 + tmp % 10;
            tmp /= 10;
        }

        x == mem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_palindrome(121));
        assert_eq!(false, Solution::is_palindrome(-121));
        assert_eq!(false, Solution::is_palindrome(10));
    }
}
