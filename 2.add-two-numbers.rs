/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add(l1, l2, 0)
    }

    pub fn add(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
        if (l1.is_none() && l2.is_none() && carry == 0) {
            return None;
        }

        let mut sum = carry;
        l1.take().map(|node| { 
            sum += node.val;
            l1 = node.next;
        });
        l2.take().map(|node| {
            sum += node.val;
            l2 = node.next;
        });

        let mut node = ListNode::new(sum % 10);
        node.next = Self::add(l1, l2, sum / 10);
            
        Some(Box::new(node))
    }
}
// @lc code=end

