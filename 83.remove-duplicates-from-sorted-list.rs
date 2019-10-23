/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
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
use std::mem;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut hmut = head.as_mut();

        while let Some(node) = hmut.take() {
            match node.next.as_mut() {
                None => (),
                Some(next) => {
                    if node.val == next.val {
                        node.next = next.next.take();
                        hmut = Some(node);
                    } else {
                        hmut = node.next.as_mut()
                    }
                }
            }
        }

        head
    }
}
// @lc code=end

