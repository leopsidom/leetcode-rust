/*
 * @lc app=leetcode id=51 lang=rust
 *
 * [51] N-Queens
 */

// @lc code=start
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let N = n as usize;

        let mut board = vec![vec!['.'; N]; N];
        let mut ans = vec![];

        let mut cols = vec![false; N];
        let mut diag = vec![false; 2 * N - 1];
        let mut anti_diag = vec![false; 2 * N - 1];

        fn helper(
            i: i32, n: i32,
            cols: &mut Vec<bool>, diag: &mut Vec<bool>, anti_diag: &mut Vec<bool>, 
            board: &mut Vec<Vec<char>>, ans: &mut Vec<Vec<String>>) {
            if i == n {
                let solution: Vec<String> = board.iter().map(|v| v.iter().collect::<String>()).collect();
                ans.push(solution);
                return;
            }

            for j in 0..(n as usize) {
                let d_idx = (i - j as i32 + n - 1) as usize;
                let a_idx = (i + j as i32) as usize;

                if !cols[j] && !diag[d_idx] && !anti_diag[a_idx] {
                    cols[j] = true;
                    diag[d_idx] = true;
                    anti_diag[a_idx] = true;
                    board[i as usize][j] = 'Q';
                    helper(i+1, n, cols, diag, anti_diag, board, ans);
                    board[i as usize][j] = '.';
                    cols[j] = false;
                    diag[d_idx] = false;
                    anti_diag[a_idx] = false;
                }
            }
        }

        helper(0, n, &mut cols, &mut diag, &mut anti_diag, &mut board, &mut ans);

        ans
    }
}
// @lc code=end

