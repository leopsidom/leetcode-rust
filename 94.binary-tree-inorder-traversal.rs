/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Self::inorder_traversal_rec(root)
        Self::inorder_traversal_iter(root)
    }

    pub fn inorder_traversal_rec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, mut ans: &mut Vec<i32>) {
            if let Some(rc_node) = root {
                let node = rc_node.borrow();
                helper(node.left.clone(), &mut ans);
                ans.push(node.val);
                helper(node.right.clone(), &mut ans);
            }
        }

        let mut ans = vec![];
        helper(root, &mut ans);
        ans
    }

    pub fn inorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![root.clone()];
        let mut node = root;

        while let Some(node_inner) = node.take() {
            node = node_inner.borrow().left.clone();
            if node.is_some() {
                stack.push(node.clone());
            }
        }

        while let Some(mut node) = stack.pop() {
            if let Some(node_inner) = node.clone() {
                ans.push(node_inner.borrow().val);
                node = node_inner.borrow().right.clone();
                if node.is_some() {
                    stack.push(node.clone());
                }
            }

            while let Some(node_inner) = node.take() {
                node = node_inner.borrow().left.clone();
                if node.is_some() {
                    stack.push(node.clone());
                }
            }
        }

        ans
    }
}
// @lc code=end

