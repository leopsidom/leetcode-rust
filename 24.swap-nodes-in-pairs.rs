/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head.as_mut() {
            None => None,
            Some(node) => {
                match node.next.take() {
                    None => head,
                    Some(mut next) => {
                        node.next = Self::swap_pairs(next.next.take());
                        next.next = head;
                        Some(next)
                    }
                }
            }
        }

    }
}
// @lc code=end

