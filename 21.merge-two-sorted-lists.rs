/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
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
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        match (l1.as_mut(), l2.as_mut()) {
            (None, None) => None,
            (Some(node1), None) => l1,
            (None, Some(node2)) => l2,
            (Some(node1), Some(node2)) => {
                if (node1.val < node2.val) {
                    node1.next = Self::merge_two_lists(node1.next.take(), l2);
                    l1
                } else {
                    node2.next = Self::merge_two_lists(l1, node2.next.take());
                    l2
                }
            }
        }
    }
}
// @lc code=end

