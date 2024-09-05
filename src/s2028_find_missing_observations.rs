// problem: https://leetcode.com/problems/find-missing-observations

pub struct Solution {}

impl Solution {
    pub fn missing_rolls(_rolls: Vec<i32>, _mean: i32, _n: i32) -> Vec<i32> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![6,6], Solution::missing_rolls(vec![3,2,4,3], 4, 2));
        assert_eq!(vec![2,3,2,2], Solution::missing_rolls(vec![1,5,6], 3, 4));
        assert_eq!(Vec::<i32>::new(), Solution::missing_rolls(vec![1,2,3,4], 6, 4));
    }
}
