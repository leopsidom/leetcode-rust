/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
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
use std::cmp;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            if let Some(rc_node) = root {
                let tree_node = rc_node.borrow();
                let (is_left_balanced, left_height) = helper(tree_node.left.clone());
                let (is_right_balanced, right_height) = helper(tree_node.right.clone());

                if is_left_balanced && is_right_balanced && (left_height - right_height).abs() <= 1 {
                    (true, cmp::max(left_height, right_height) + 1)
                } else {
                    (false, 0)
                }
            } else {
                (true, 0)
            }
        }

        let (ans, _) = helper(root);
        ans
    }
}
// @lc code=end

