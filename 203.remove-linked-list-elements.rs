/*
 * @lc app=leetcode id=203 lang=rust
 *
 * [203] Remove Linked List Elements
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head.as_mut() {
            None => None,
            Some(node) => {
                let next = node.next.take();
                if node.val != val {
                    node.next = Self::remove_elements(next, val);
                    head
                } else {
                    Self::remove_elements(next, val)
                }
            }
        }
    }
}
// @lc code=end

