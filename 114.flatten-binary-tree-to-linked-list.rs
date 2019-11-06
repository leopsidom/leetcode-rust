/*
 * @lc app=leetcode id=114 lang=rust
 *
 * [114] Flatten Binary Tree to Linked List
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {

        if let Some(rc_node) = root {
            Self::flatten(&mut rc_node.borrow_mut().left);
            Self::flatten(&mut rc_node.borrow_mut().right);

            let mut left = rc_node.borrow_mut().left.take();
            if left.is_some() {
                let right = rc_node.borrow().right.clone();
                rc_node.borrow_mut().right = left.clone();

                let mut left_ref = left.clone();
                while let Some(next) = left_ref.take() {
                    if next.borrow().right.is_some() {
                        left_ref = next.borrow_mut().right.clone();
                    } else {
                        next.borrow_mut().right = right;
                        break;
                    }
                }
            }
        }

    }
}
// @lc code=end

