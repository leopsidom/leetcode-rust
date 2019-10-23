/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut prev = None;
        let mut curr = head.take();

        while let Some(mut curr_inner) = curr.take() {
            let next = curr_inner.next.take();
            curr_inner.next = prev.take();
            prev = Some(curr_inner);
            curr = next;
        }

        return prev;
    }
}
// @lc code=end

