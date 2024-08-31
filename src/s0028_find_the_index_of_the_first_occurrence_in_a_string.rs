// problem: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, Solution::str_str("sadbutsad".to_string(), "sad".to_string()));
        assert_eq!(-1, Solution::str_str("leetcode".to_string(), "leeto".to_string()));
    }
}
