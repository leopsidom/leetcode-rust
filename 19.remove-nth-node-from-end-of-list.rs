/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        
        let mut N = 0;
        let mut href = head.as_ref();

        while href.is_some() {
            href = match href.take() {
                None => None,
                Some(node) => node.next.as_ref()
            };
            N += 1;
        }
        
        if (N == n) {
            return match head.as_mut() {
                None => None,
                Some(mut node) => node.next.take()
            }
        }
        N -= n;

        let mut hmut = head.as_mut();

        while N > 1 {
            hmut = match hmut.take() {
                None => None,
                Some(node) => node.next.as_mut()
            };
            N -= 1;
        }

        match hmut.take() {
            None => (),
            Some(prev) => {
                match prev.next.take() {
                    None => (),
                    Some(mut next) => {
                        prev.next = next.next.take()
                    }
                }
            }
        }

        head
    }
}
// @lc code=end

