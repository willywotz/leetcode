// problem: https://leetcode.com/problems/length-of-last-word/

pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(_s: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, Solution::length_of_last_word("Hello World".to_string()));
        assert_eq!(4, Solution::length_of_last_word("   fly me   to   the moon  ".to_string()));
        assert_eq!(6, Solution::length_of_last_word("luffy is still joyboy".to_string()));
    }
}
