// problem: https://leetcode.com/problems/valid-parentheses/

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let opening = HashMap::from([(']', '['), (')', '('), ('}', '{')]);

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                _ => {
                    if stack.iter().last() == opening.get(&c) {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
        assert_eq!(false, Solution::is_valid("(]".to_string()));
        assert_eq!(true, Solution::is_valid("([])".to_string()));
    }
}
