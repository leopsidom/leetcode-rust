/*
 * @lc app=leetcode id=606 lang=rust
 *
 * [606] Construct String from Binary Tree
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
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        match t {
            None => "".to_owned(),
            Some(rc_node) => {
                let tree_node = rc_node.borrow();
                let left = Self::tree2str(tree_node.left.clone());
                let right = Self::tree2str(tree_node.right.clone());

                let mut ans = String::new();
                ans.push_str(&tree_node.val.to_string());

                if left != "" || right != "" {
                    ans.push('(');
                    ans.push_str(&left);
                    ans.push(')');
                }

                if right != "" {
                    ans.push('(');
                    ans.push_str(&right);
                    ans.push(')');
                }

                ans
            }
        }
    }
}
// @lc code=end

