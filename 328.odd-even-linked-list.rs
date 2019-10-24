/*
 * @lc app=leetcode id=328 lang=rust
 *
 * [328] Odd Even Linked List
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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h1 = Some(Box::new(ListNode::new(0)));
        let mut h2 = Some(Box::new(ListNode::new(0)));

        let mut hmut1 = h1.as_mut();
        let mut hmut2 = h2.as_mut();
        let mut cnt = 1;

        while let Some(mut node) = head.take() {
            head = node.next.take();
            if (cnt % 2 == 1) {
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
            cnt += 1;
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

