/*
 * @lc app=leetcode id=572 lang=rust
 *
 * [572] Subtree of Another Tree
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
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(s_rc_node) = s.clone() {
            let tree_node = s_rc_node.borrow();
            Self::is_sametree(s.clone(), t.clone()) ||
            Self::is_subtree(tree_node.left.clone(), t.clone()) ||
            Self::is_subtree(tree_node.right.clone(), t.clone())
        } else {
            return t.is_none();
        }
    }

    pub fn is_sametree(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(x), Some(y)) => {
                let xnode = x.borrow();
                let ynode = y.borrow();
                xnode.val == ynode.val && 
                Self::is_sametree(xnode.left.clone(), ynode.left.clone()) &&
                Self::is_sametree(xnode.right.clone(), ynode.right.clone())
            }
        }
    }
}
// @lc code=end

