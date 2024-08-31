// problem: https://leetcode.com/problems/add-binary/

pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = Vec::new();
        let mut carry = 0u8;

        let mut a_bytes = a.as_bytes().to_vec();
        let mut b_bytes = b.as_bytes().to_vec();

        a_bytes.reverse();
        b_bytes.reverse();

        for i in 0..a.len().max(b.len()) {
            carry += if i < a_bytes.len() { a_bytes[i] - b'0' } else { 0 };
            carry += if i < b_bytes.len() { b_bytes[i] - b'0' } else { 0 };
            result.push((carry % 2) as u8 + b'0');
            carry /= 2;
        }

        if carry == 1 {
            result.push(b'1');
        }

        result.reverse();
        String::from_utf8(result).unwrap()
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
