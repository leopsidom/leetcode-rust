/*
 * @lc app=leetcode id=86 lang=rust
 *
 * [86] Partition List
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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut h1 = Some(Box::new(ListNode::new(0)));
        let mut h2 = Some(Box::new(ListNode::new(0)));

        let mut hmut1 = h1.as_mut();
        let mut hmut2 = h2.as_mut();

        while let Some(mut node) = head.take() {
            head = node.next.take();
            if (node.val < x) {
                if let Some(mut h1_inner) = hmut1.take() {
                    h1_inner.next = Some(node);
                    hmut1 = h1_inner.next.as_mut();
                }
            } else {
                if let Some(mut h2_inner) = hmut2.take() {
                    h2_inner.next = Some(node);
                    hmut2 = h2_inner.next.as_mut();
                }
            }
        }

        if let Some(mut h2_inner) = hmut2.take() {
            h2_inner.next = None;
        }
        if let Some(mut h1_inner) = hmut1.take() {
            if let Some(mut h2_head) = h2.take() {
                h1_inner.next = h2_head.next.take();
            }
        }

        h1.take().and_then(|node| node.next)
    }
}
// @lc code=end

