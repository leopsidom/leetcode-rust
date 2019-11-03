/*
 * @lc app=leetcode id=95 lang=rust
 *
 * [95] Unique Binary Search Trees II
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            let mut ans = vec![];

            if start > end {
                ans.push(None);
                return ans;
            }

            for i in start..end+1 {
                
                for left in helper(start, i-1) {
                    for right in helper(i+1, end) {
                        let tree_node = Rc::new(RefCell::new(TreeNode::new(i)));
                        tree_node.borrow_mut().left = left.clone();
                        tree_node.borrow_mut().right = right.clone();
                        // if let Some(node) = right.clone() {
                        //     println!("i: {}, right: {}", i, node.borrow().val);
                        // }
                        ans.push(Some(tree_node));
                    }
                }
            }

            ans
        }

        if n == 0 { return vec![]; }
        helper(1, n)
    }
}
// @lc code=end

