/*
 * @lc app=leetcode id=617 lang=rust
 *
 * [617] Merge Two Binary Trees
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
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (Some(rn1), Some(rn2)) => {
                let tn1 = rn1.borrow();
                let tn2 = rn2.borrow();

                let mut tn = TreeNode::new(tn1.val + tn2.val);
                tn.left = Self::merge_trees(tn1.left.clone(), tn2.left.clone());
                tn.right = Self::merge_trees(tn1.right.clone(), tn2.right.clone());

                Some(Rc::new(RefCell::new(tn)))
            },
            (Some(rn1), None) => Some(rn1.clone()),
            (None, Some(rn2)) => Some(rn2.clone()),
            (None, None) => None,
        }
    }
}
// @lc code=end

