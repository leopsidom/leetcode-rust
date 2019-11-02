/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {

        match (p, q) {
            (None, None) => true,
            (Some(p_node_rc), Some(q_node_rc)) => {
                let p_node = p_node_rc.borrow();
                let q_node = q_node_rc.borrow();

                if p_node.val != q_node.val {
                    false
                } else {
                    Self::is_same_tree(p_node.left.clone(), q_node.left.clone()) && 
                    Self::is_same_tree(p_node.right.clone(), q_node.right.clone())
                }
            },
            _ => false
        }
    }
}
// @lc code=end

