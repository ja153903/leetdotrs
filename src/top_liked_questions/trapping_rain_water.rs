#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_left = height[0];
        let mut max_right = height[height.len() - 1];
        let mut result = 0;

        let mut i: i32 = 0;
        let mut j: i32 = height.len() as i32 - 1;

        while i < j {
            if height[i as usize] < height[j as usize] {
                if max_left < height[i as usize] {
                    max_left = height[i as usize];
                } else {
                    result += max_left - height[i as usize];
                    i += 1;
                }
            } else {
                if max_right < height[j as usize] {
                    max_right = height[j as usize];
                } else {
                    result += max_right - height[j as usize];
                    j -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_trapping_rain_water_case1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let expected = 6;

        let result = Solution::trap(height);

        assert_eq!(
            result, expected,
            "Expected: {}, but got {}",
            expected, result
        );
    }
}
