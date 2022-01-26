#![allow(dead_code)]

struct Solution;

use std::cmp;

type Intervals = Vec<Vec<i32>>;

impl Solution {
    pub fn merge(mut intervals: Intervals) -> Intervals {
        let mut result = Vec::new();

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        for interval in intervals {
            if result.is_empty() {
                result.push(interval);
            } else {
                if let Some(top) = result.last_mut() {
                    if top[1] < interval[0] {
                        result.push(interval);
                    } else {
                        top[1] = cmp::max(top[1], interval[1]);
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{Intervals, Solution};

    #[test]
    pub fn test_merge_intervals_case1() {
        let intervals: Intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected: Intervals = vec![vec![1, 6], vec![8, 10], vec![15, 18]];

        let result: Intervals = Solution::merge(intervals);

        assert_eq!(
            result, expected,
            "Expected: {:?}, but got {:?}",
            result, expected
        );
    }
}
