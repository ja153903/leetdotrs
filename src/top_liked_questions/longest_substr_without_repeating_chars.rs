#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_length_of_longest_substring_case1() {
        let s = String::from("abcabcbb");
        let expected: i32 = 3;

        let result = Solution::length_of_longest_substring(s);

        assert_eq!(
            result, expected,
            "Expected: {}, but got {}",
            expected, result
        );
    }
}
