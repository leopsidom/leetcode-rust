/*
 * @lc app=leetcode id=222 lang=rust
 *
 * [222] Count Complete Tree Nodes
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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // Self::count_naive(root)
        Self::count_height(root)
    }

    pub fn count_naive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc_node) => {
                let tree_node = rc_node.borrow();
                Self::count_naive(tree_node.left.clone()) +
                Self::count_naive(tree_node.right.clone()) + 1
            },
            None => 0
        }
    }

    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc_node) => Self::height(rc_node.borrow().left.clone()) + 1,
            None => -1
        }
    }

    pub fn count_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc_node) => {
                let h = Self::height(Some(rc_node.clone()));
                if Self::height(rc_node.borrow().right.clone()) == h - 1 {
                    (1 << h) + Self::count_height(rc_node.borrow().right.clone())
                } else {
                    (1 << (h - 1)) + Self::count_height(rc_node.borrow().left.clone())
                }
            },
            None => 0
        }
    }
}
// @lc code=end

