/*
 * @lc app=leetcode id=501 lang=rust
 *
 * [501] Find Mode in Binary Search Tree
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
use std::collections::HashMap;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut counter = HashMap::new();
        let mut max_count = 0;

        fn in_order(root: Option<Rc<RefCell<TreeNode>>>, counter: &mut HashMap<i32, i32>, max_count: &mut i32) {
            if let Some(rc_node) = root {
                let tree_node = rc_node.borrow();
                let left_node = tree_node.left.clone();
                
                let val = tree_node.val;
                let count = counter.entry(val).or_insert(0);
                *count += 1;

                let right_node = tree_node.right.clone();

            }
        }
    }
}
// @lc code=end

