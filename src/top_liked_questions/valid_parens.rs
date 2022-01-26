#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' => {
                    stack.push(')');
                }
                '{' => {
                    stack.push('}');
                }
                '[' => {
                    stack.push(']');
                }
                _ => {
                    if stack.is_empty() {
                        return false;
                    }

                    if let Some(last) = stack.pop() {
                        if last != ch {
                            return false;
                        }
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_valid_parens_case1() {
        let case = String::from("()");

        assert!(Solution::is_valid(case));
    }

    #[test]
    pub fn test_valid_parens_case2() {
        let case = String::from("()[]{}");

        assert!(Solution::is_valid(case));
    }

    #[test]
    pub fn test_valid_parens_case3() {
        let case = String::from("(]");

        assert!(!Solution::is_valid(case));
    }
}
