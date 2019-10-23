/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
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
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut max_heap = BinaryHeap::new();
        let N = lists.len();

        for i in 0..N {
            if let Some(list_node) = &lists[i] {
                max_heap.push((-list_node.val, i));
            }
        }

        Self::merge_k_lists_helper(lists, max_heap)
    }

    pub fn merge_k_lists_helper(mut lists: Vec<Option<Box<ListNode>>>, mut max_heap: BinaryHeap<(i32, usize)>) -> Option<Box<ListNode>> {
        if let Some((val, i)) = max_heap.pop() {
            let mut head = lists[i].take();

            match head.as_mut() {
                None => None,
                Some(node) => {
                    let next = node.next.take();
                    if let Some(node1) = &next {
                        max_heap.push((-node1.val, i));
                    }
                    lists[i] = next;
                    node.next = Self::merge_k_lists_helper(lists, max_heap);
                    head
                }
            }
        } else {
            None
        }
    }
}
// @lc code=end

