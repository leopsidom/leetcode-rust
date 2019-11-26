/*
 * @lc app=leetcode id=61 lang=rust
 *
 * [61] Rotate List
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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let N = Self::length(&head);
        if N == 0 || k % N == 0 { return head; }

        let mut M = N - k % N;

        let mut hmut = head.as_mut();
        while M > 1 {
            hmut = hmut.unwrap().next.as_mut();
            M -= 1;
        }

        let mut ans = hmut.unwrap().next.take();

        hmut = ans.as_mut();

        while let Some(node) = hmut {
            if node.next.is_some() {
                hmut = node.next.as_mut();
            } else {
                node.next = head;
                break;
            }
        }

        ans
    }

    pub fn length(mut head: &Option<Box<ListNode>>) -> i32 {
        let mut N = 0;

        while let Some(node) = head {
            head = &node.next;
            N += 1;
        }

        N
    }
}
// @lc code=end

