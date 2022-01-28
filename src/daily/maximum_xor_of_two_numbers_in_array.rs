#![allow(dead_code)]

struct Solution;

use std::cmp;

// TODO: No clue how to do this:
impl Solution {
    pub fn find_maximum_xor(_nums: Vec<i32>) -> i32 {
        0
    }

    pub fn find_maximum_xor_bf(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let n = nums.len();

        for i in 0..n {
            for j in (0..n).skip(i + 1) {
                result = cmp::max(result, nums[i] ^ nums[j]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    //#[test]
    pub fn test_find_maximum_xor_case1() {
        let nums = vec![3, 10, 5, 25, 2, 8];
        let expected = 28;

        let solution = Solution::find_maximum_xor(nums);

        assert_eq!(
            expected, solution,
            "Expected: {}, but Got: {}",
            expected, solution
        );
    }

    #[test]
    pub fn test_find_maximum_xor_bf_case1() {
        let nums = vec![3, 10, 5, 25, 2, 8];
        let expected = 28;

        let solution = Solution::find_maximum_xor_bf(nums);

        assert_eq!(
            expected, solution,
            "Expected: {}, but Got: {}",
            expected, solution
        );
    }
}
