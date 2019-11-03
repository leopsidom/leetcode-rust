/*
 * @lc app=leetcode id=107 lang=rust
 *
 * [107] Binary Tree Level Order Traversal II
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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![]; }

        let mut nodes = vec![root.clone()];
        let mut ans = vec![];

        while nodes.len() > 0 {
            let mut tmp = vec![];
            let mut vals = vec![];

            for node in nodes {
                if let Some(node_inner) = node {
                    let tree_node = node_inner.borrow();
                    vals.push(tree_node.val);
                    if tree_node.left.is_some() {
                        tmp.push(tree_node.left.clone());
                    }
                    if tree_node.right.is_some() {
                        tmp.push(tree_node.right.clone());
                    }
                }
            }

            nodes = tmp;
            ans.push(vals);
        }

        ans.reverse();
        ans
    }
}
// @lc code=end

