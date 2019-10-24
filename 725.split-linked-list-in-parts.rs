/*
 * @lc app=leetcode id=725 lang=rust
 *
 * [725] Split Linked List in Parts
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
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut N = 0;
        let mut href = root.as_ref();

        while let Some(node) = href.take() {
            href = node.next.as_ref();
            N += 1;
        }

        let mut ans = vec![None; k as usize];
        let l = (N / k) as usize;
        let m = N as usize - l * k as usize;
        let n = k as usize - m;

        // println!("N: {}, l: {}, m: {}, n: {}, k: {}", N, l, m, n, k);
        let mut head = root;
        for i in 0..m {

            ans[i] = head.take();

            let mut hmut = ans[i].as_mut();
            for j in 0..l {
                if let Some(mut node) = hmut.take() {
                    hmut = node.next.as_mut();
                }
            }


            if let Some(mut node) = hmut.take() {
                head = node.next.take();
            }
        }

        if (l >= 1) {
            for i in m..(k as usize) {

                ans[i] = head.take();

                let mut hmut = ans[i].as_mut();
                for j in 0..(l-1) {
                    if let Some(mut node) = hmut.take() {
                        hmut = node.next.as_mut();
                    }
                }


                if let Some(mut node) = hmut.take() {
                    head = node.next.take();
                }
            }
        }

        ans

    }
}
// @lc code=end

