/*
 * @lc app=leetcode id=123 lang=rust
 *
 * [123] Best Time to Buy and Sell Stock III
 */

// @lc code=start
use std::i32;
use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        Self::naive_sol(&prices)
    }

    pub fn naive_sol(prices: &Vec<i32>) -> i32 {
        let N = prices.len();
        if N < 2 { return 0; }

        let mut one_transaction_max = 0;
        let mut prefix_min = prices[0];
        let mut ans = 0;

        for i in 0..N {
            prefix_min = cmp::min(prefix_min, prices[i]);
            one_transaction_max = cmp::max(one_transaction_max, prices[i] - prefix_min);

            if i < N - 2 {
                let mut prefix_min2 = prices[i+1];
                let mut max_profit_since_i = 0;
                for j in i+1..N {
                    prefix_min2 = cmp::min(prefix_min2, prices[j]);
                    max_profit_since_i = cmp::max(max_profit_since_i, prices[j] - prefix_min2);
                }            
                ans = cmp::max(ans, one_transaction_max + max_profit_since_i);
            }
        }

        cmp::max(one_transaction_max, ans)
    }
}
// @lc code=end

