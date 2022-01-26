#![allow(dead_code)]

struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        if root1.is_none() && root2.is_none() {
            return Vec::new();
        }

        let mut res1: Vec<i32> = Vec::new();
        let mut res2: Vec<i32> = Vec::new();
        let mut res: Vec<i32> = Vec::new();

        Solution::inorder(root1, &mut res1);
        Solution::inorder(root2, &mut res2);

        if res1.is_empty() {
            return res2;
        }

        if res2.is_empty() {
            return res1;
        }

        let n = res1.len() - 1;
        let m = res2.len() - 1;

        let mut i = 0;
        let mut j = 0;

        while i <= n && j <= m {
            if res1[i] < res2[j] {
                res.push(res1[i]);
                i += 1;
            } else {
                res.push(res2[j]);
                j += 1;
            }
        }

        while i <= n {
            res.push(res1[i]);
            i += 1;
        }

        while j <= m {
            res.push(res2[j]);
            j += 1;
        }

        res
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        if let Some(node) = root {
            Solution::inorder(node.borrow().left.clone(), result);
            result.push(node.borrow().val);
            Solution::inorder(node.borrow().right.clone(), result);
        }
    }
}
