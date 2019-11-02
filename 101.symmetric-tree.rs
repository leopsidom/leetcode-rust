/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() { return true; }

        let mut nodes = vec![root];

        while true {
            let mut tmp = vec![];
            let mut has_some = false;

            for node in nodes {
                if let Some(rc_node) = node {
                    let tree_node = rc_node.borrow();
                    tmp.push(tree_node.left.clone());
                    tmp.push(tree_node.right.clone());
                    has_some = true;
                } else {
                    tmp.push(None);
                    tmp.push(None);
                }
            }

            if !has_some { break; }

            let N = tmp.len();

            if N % 2 != 0 {
                return false;
            }

            for i in 0..N/2 {
                let is_sym = match (tmp[i].clone(), tmp[N-i-1].clone()) {
                    (Some(left), Some(right)) => {
                        left.borrow().val == right.borrow().val
                    },
                    (None, None) => true,
                    _ => false
                };
                
                if !is_sym { return false; }
            }

            nodes = tmp;
        }

        return true;
    }
}
// @lc code=end

