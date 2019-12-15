/*
 * @lc app=leetcode id=230 lang=rust
 *
 * [230] Kth Smallest Element in a BST
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if root.is_none() { return 0; }
        let mut nodes = vec![];

        while let Some(rc_node) = root {
            nodes.push(Some(rc_node.clone()));
            root = rc_node.borrow().left.clone();
        }
        
        let mut i = 0;
        loop {
            root = nodes.pop().unwrap();
            i += 1;
            if i == k {
                let val = root.unwrap().borrow().val;
                return val;
            }

            root = root.unwrap().borrow().right.clone();
            while let Some(rc_node) = root {
                nodes.push(Some(rc_node.clone()));
                root = rc_node.borrow().left.clone();
            }
        }

        return 0;
    }
}
// @lc code=end

