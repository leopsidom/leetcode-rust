/*
 * @lc app=leetcode id=216 lang=rust
 *
 * [216] Combination Sum III
 */

// @lc code=start
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut combs = vec![vec![]];
        let mut ans = vec![];

        for num in 1..10 {
            let N = combs.len();
            let mut tmp = vec![];

            combs.drain(0..N).for_each(|mut comb| {
                tmp.push(comb.clone());

                if comb.len() < k as usize {
                    comb.push(num);
                }

                if comb.len() == k as usize {
                    if comb.iter().sum::<i32>() == n {
                        ans.push(comb);
                    }
                } else {
                    tmp.push(comb);
                }
            });

            // println!("{:?}", tmp);
            combs = tmp;
        }

        ans
    }
}
// @lc code=end

