#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new();

        for (i, &val) in nums.iter().enumerate() {
            if let Some(&j) = mp.get(&(target - val)) {
                return vec![j, i as i32];
            }

            mp.insert(val, i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_two_sum_case1() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![0, 1]);
    }
}
