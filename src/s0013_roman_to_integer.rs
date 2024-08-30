// problem: https://leetcode.com/problems/roman-to-integer/

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = HashMap::from([
            (b'\0', 0), (b'I', 1), (b'V', 5), (b'X', 10), (b'L', 50),
            (b'C', 100), (b'D', 500), (b'M', 1000)
        ]);

        let mut ret = 0;
        let mut tmp = b'\0';

        for c in s.bytes().rev() {
            let v = map[&c];
            if map[&tmp] <= v {
                ret += v;
            } else {
                ret -= v;
            }
            tmp = c;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
