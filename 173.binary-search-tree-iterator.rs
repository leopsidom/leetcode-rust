/*
 * @lc app=leetcode id=173 lang=rust
 *
 * [173] Binary Search Tree Iterator
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

struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];

        while let Some(node) = root.clone() {
            stack.push(root.clone());
            root = node.borrow().left.clone();
        }

        BSTIterator { stack }
    }
    
    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let mut root = self.stack.pop().unwrap();
        let node = root.unwrap();
        let ans = node.borrow().val;
        root = node.borrow().right.clone();

        while let Some(node) = root.clone() {
            self.stack.push(root.clone());
            root = node.borrow().left.clone();
        }

        ans
    }
    
    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// @lc code=end

