#![allow(dead_code)]

use crate::data_structures::ListNode;

struct Solution;

// For linked list problems, we should opt to use
// recursive solutions.
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::merge(
            l1,
            l2,
            0,
            ListNode {
                val: -1,
                next: None,
            },
        )
    }

    pub fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
        mut carry: i32,
        mut result: ListNode,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            None
        } else {
            if let Some(n1) = l1 {
                carry += n1.val;
                l1 = n1.next;
            }

            if let Some(n2) = l2 {
                carry += n2.val;
                l2 = n2.next;
            }

            result.val = carry % 10;
            result.next = Solution::merge(
                l1,
                l2,
                carry / 10,
                ListNode {
                    val: -1,
                    next: None,
                },
            );

            Some(Box::new(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::data_structures::linked_list_utils;

    #[test]
    pub fn test_add_two_numbers_test_case1() {
        let v1 = vec![2, 4, 3];
        let v2 = vec![5, 6, 4];

        let l1 = linked_list_utils::create_linked_list_from_vec(v1);
        let l2 = linked_list_utils::create_linked_list_from_vec(v2);

        let result = Solution::add_two_numbers(l1, l2);

        let v_result = linked_list_utils::create_vec_from_linked_list(result);

        assert_eq!(v_result, vec![7, 0, 8]);
    }
}
