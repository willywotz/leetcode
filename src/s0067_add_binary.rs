// problem: https://leetcode.com/problems/add-binary/

pub struct Solution {}

impl Solution {
    pub fn add_binary(_a: String, _b: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("100".to_string(), Solution::add_binary("11".to_string(), "1".to_string()));
        assert_eq!("10101".to_string(), Solution::add_binary("1010".to_string(), "1011".to_string()));
    }
}
