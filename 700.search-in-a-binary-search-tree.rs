/*
 * @lc app=leetcode id=700 lang=rust
 *
 * [700] Search in a Binary Search Tree
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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let treeNode = node.borrow();
            if treeNode.val == val {
                return Some(Rc::clone(&node));
            } else if treeNode.val < val {
                if let Some(right) = treeNode.right.as_ref() {
                    return Self::search_bst(Some(Rc::clone(right)), val);
                }
            } else {
                if let Some(left) = treeNode.left.as_ref() {
                    return Self::search_bst(Some(Rc::clone(left)), val);
                }
            }
        }
        None
    }
}
// @lc code=end

