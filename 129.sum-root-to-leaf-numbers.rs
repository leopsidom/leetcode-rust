/*
 * @lc app=leetcode id=129 lang=rust
 *
 * [129] Sum Root to Leaf Numbers
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, presum: i32) -> i32 {
            match root {
                Some(rc_node) => {
                    let tree_node = rc_node.borrow();
                    if tree_node.left.is_none() && tree_node.right.is_none() {
                        presum * 10 + tree_node.val
                    } else {
                        helper(&tree_node.left, presum * 10 + tree_node.val) +
                        helper(&tree_node.right, presum * 10 + tree_node.val)
                    }
                },
                None => 0
            }
        }

        helper(&root, 0)
    }
}
// @lc code=end

