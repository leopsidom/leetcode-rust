/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];

        fn preorder(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            match root {
                Some(rc_node) => {
                    let tree_node = rc_node.borrow();
                    vals.push(tree_node.val);
                    preorder(tree_node.left.clone(), vals);
                    preorder(tree_node.right.clone(), vals);
                },
                None => ()
            }
        }

        preorder(root, &mut ans);
        ans
    }
}
// @lc code=end

