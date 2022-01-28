#![allow(dead_code)]

struct Solution;

use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_so_far = nums[0];
        let mut max_result = nums[0];

        for &num in nums.iter().skip(1) {
            max_so_far = cmp::max(max_so_far + num, num);
            max_result = cmp::max(max_result, max_so_far);
        }

        max_result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_maximum_subarray_case1() {
        let nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let expected: i32 = 6;

        let result: i32 = Solution::max_sub_array(nums);

        assert_eq!(
            result, expected,
            "Expected: {}, but Got: {}",
            expected, result
        );
    }

    #[test]
    pub fn test_maximum_subarray_case2() {
        let nums: Vec<i32> = vec![5, 4, -1, 7, 8];
        let expected: i32 = 23;

        let result: i32 = Solution::max_sub_array(nums);

        assert_eq!(
            result, expected,
            "Expected: {}, but Got: {}",
            expected, result
        );
    }
}
