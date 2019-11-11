/*
 * @lc app=leetcode id=654 lang=rust
 *
 * [654] Maximum Binary Tree
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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.len() == 0 { return None; }

            let mut maxidx = 0;
            for i in 0..nums.len() {
                if nums[i] > nums[maxidx] {
                    maxidx = i;
                }
            }

            let mut tree_node = TreeNode::new(nums[maxidx]);
            tree_node.left = helper(&nums[..maxidx]);
            tree_node.right = helper(&nums[maxidx+1..]);

            Some(Rc::new(RefCell::new(tree_node)))
        }

        helper(&nums[..])
    }
}
// @lc code=end

