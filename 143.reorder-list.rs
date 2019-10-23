/*
 * @lc app=leetcode id=143 lang=rust
 *
 * [143] Reorder List
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
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        
        let mut href = head.as_ref();
        let mut N = 0;

        while let Some(node) = href.take() {
            href = node.next.as_ref();
            N += 1;
        }

        let mut hmut = head.as_mut();
        let mut cnt = 1;

        while cnt < (N + 1) / 2 {
            if let Some(node) = hmut.take() {
                hmut = node.next.as_mut();
                cnt += 1;
            } else {
                break;
            }
        }

        match hmut.take() {
            None => (),
            Some(node) => {
                let tail = Self::reverse(node.next.take());
                if let Some(node) = head {
                    node.next = Self::merge_two_lists(tail, node.next.take());
                }
            }
        }

    }

    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        match (l1.as_mut(), l2.as_mut()) {
            (None, None) => None,
            (Some(node1), None) => l1,
            (None, Some(node2)) => l2,
            (Some(node1), Some(node2)) => {
                node1.next = Self::merge_two_lists(l2, node1.next.take());
                l1
            }
        }
    }

    pub fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_inner) = curr.take() {
            curr = curr_inner.next.take();
            curr_inner.next = prev.take();
            prev = Some(curr_inner);
        }

        prev
    }
}
// @lc code=end

