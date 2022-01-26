#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub mod linked_list_utils {
    use super::{linked_list_utils, ListNode};

    pub fn create_linked_list_from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        linked_list_utils::_create_linked_list_from_vec_rec(
            nums,
            0 as usize,
            ListNode {
                val: -1,
                next: None,
            },
        )
    }

    fn _create_linked_list_from_vec_rec(
        nums: Vec<i32>,
        idx: usize,
        mut node: ListNode,
    ) -> Option<Box<ListNode>> {
        if idx == nums.len() {
            None
        } else {
            node.val = nums[idx];
            node.next = linked_list_utils::_create_linked_list_from_vec_rec(
                nums,
                idx + 1,
                ListNode {
                    val: -1,
                    next: None,
                },
            );

            Some(Box::new(node))
        }
    }

    pub fn create_vec_from_linked_list(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();

        while let Some(n) = head {
            v.push(n.val);
            head = n.next;
        }

        v
    }
}

// TODO: Create a mod for tree_utils:
// Functions to create:
// pub fn create_binary_tree_from_vec(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>
// pub fn in_order(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>)
// pub fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>)
// pub fn post_order(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>)
// pub fn level_order(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>)
