/*
 * @lc app=leetcode id=92 lang=rust
 *
 * [92] Reverse Linked List II
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
    pub fn reverse_between(mut head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if m == 1 {
            return Self::reverse_first_n(head, n);
        }

        let mut hmut = head.as_mut();
        let mut curr = 1;

        while curr < m - 1 {
            if let Some(node) = hmut.take() {
                hmut = node.next.as_mut();
            }
            curr += 1;
        }
        
        if let Some(node) = hmut.take() {
            node.next = Self::reverse_first_n(node.next.take(), n - m + 1);
        }

        head

    }

    pub fn reverse_first_n(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if (n <= 1) {
            return head;
        }

        let mut cnt = 0;
        let mut prev = None;
        let mut curr = head.take();
        
        while cnt < n {
            if let Some(mut curr_inner) = curr.take() {
                curr = curr_inner.next.take();
                curr_inner.next = prev.take();
                prev = Some(curr_inner);
            } else {
                break;
            }
            cnt += 1;
        }

        let mut href = prev.as_mut();

        while let Some(node) = href.take() {
            if (node.next.is_some()) {
                href = node.next.as_mut();
            } else {
                href = Some(node);
                break;
            }
        }

        if let Some(node) = href.take() {
            node.next = curr;
        }

        prev
    }
}
// @lc code=end

