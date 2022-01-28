#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
use std::cmp;

impl Solution {
    // 3. Longest Substring Without Repeating Characters
    //
    // Given a string s, find the length of the longest substring
    // without repeating characters
    //
    // === Approach ===
    // Keep track of starting index that we'll move along 
    // each time we find a repeated character.
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut hashmap: HashMap<char, usize> = HashMap::new();
        let mut start: usize = 0;
        let mut len: usize = 0;

        for (i, c) in s.char_indices() {
            if let Some(k) = hashmap.get(&c) {
                start = cmp::max(start, k + 1);
            }

            hashmap.insert(c, i);
            len = cmp::max(len, i - start + 1);
        }

        len as i32
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
