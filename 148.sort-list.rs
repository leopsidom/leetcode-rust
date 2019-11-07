/*
 * @lc app=leetcode id=148 lang=rust
 *
 * [148] Sort List
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let N = Self::length(&head);
        Self::sort_helper(head, N)
    }

    pub fn sort_helper(mut head: Option<Box<ListNode>>, len: u32) -> Option<Box<ListNode>> {
        if len <= 1 { return head; }
        let mid = len / 2;

        let mut hmut = head.as_mut();
        let mut i = 1;

        while i < mid {
            hmut = hmut.unwrap().next.as_mut();
            i += 1;
        }

        // println!("len: {}, mid: {}", len, mid);
        Self::merge(
            Self::sort_helper(hmut.unwrap().next.take(), len - mid),
            Self::sort_helper(head, mid)
        )   
    }

    pub fn length(head: &Option<Box<ListNode>>) -> u32 {
        let mut href = head.as_ref();
        let mut N = 0;

        while let Some(node) = href {
            href = node.next.as_ref();
            N += 1;
        }

        N
    }

    pub fn merge(mut h1: Option<Box<ListNode>>, mut h2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (h1.as_mut(), h2.as_mut()) {
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge(node1.next.take(), h2);
                    h1
                } else {
                    node2.next = Self::merge(h1, node2.next.take());
                    h2
                }
            },
            (Some(node1), None) => h1,
            (None, Some(node2)) => h2,
            (None, None) => None
        }
    }
}
// @lc code=end

