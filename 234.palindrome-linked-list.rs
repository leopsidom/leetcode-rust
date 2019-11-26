/*
 * @lc app=leetcode id=234 lang=rust
 *
 * [234] Palindrome Linked List
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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() { return true; }
        
        let mut N = 0;
        let mut href = head.as_ref();

        while let Some(node) = href {
            href = node.next.as_ref();
            N += 1;
        }

        let mut hmut = head.as_mut();
        let mut mid = N / 2 - 1;

        while mid > 0 {
            hmut = hmut.unwrap().next.as_mut();
            mid -= 1;
        }

        let mut prev = None;
        let mut next = hmut.unwrap().next.take();

        while let Some(mut next_inner) = next {
            next = next_inner.next.take();
            next_inner.next = prev.take();
            prev = Some(next_inner);
        }

        let mut h1 = &head;
        let mut h2 = &prev;

        loop {
            let (not_equal, should_break) = match (h1.as_ref(), h2.as_ref()) {
                (Some(h1_inner), Some(h2_inner)) => {
                    h1 = &h1_inner.next;
                    h2 = &h2_inner.next;
                    (h1_inner.val != h2_inner.val, false)
                },
                _ => (false, true)
            };

            if should_break { break; }
            if not_equal { return false; }
        }

        return true;

    }
}
// @lc code=end

