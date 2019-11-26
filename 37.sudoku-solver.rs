/*
 * @lc app=leetcode id=37 lang=rust
 *
 * [37] Sudoku Solver
 */

// @lc code=start
use std::collections::HashSet;
use std::char;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let s: HashSet<char> = (1..10).map(|x| char::from_digit(x, 10).unwrap()).collect();

        let mut row_choices = vec![s.clone(); 9];
        let mut col_choices = vec![s.clone(); 9];
        let mut sqr_choices = vec![s.clone(); 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    row_choices[i].remove(&board[i][j]);
                    col_choices[j].remove(&board[i][j]);
                    sqr_choices[3 * (i / 3) + (j / 3)].remove(&board[i][j]);
                }
            }
        }

        // println!("row: {:?}", row_choices);
        // println!("col: {:?}", col_choices);
        // println!("sqr: {:?}", sqr_choices);

        fn back_track(i: usize, j: usize, board: &mut Vec<Vec<char>>,
            row_choices: &mut Vec<HashSet<char>>, 
            col_choices: &mut Vec<HashSet<char>>, 
            sqr_choices: &mut Vec<HashSet<char>>
        ) -> bool {
            if i == 9 { return true; }
            else if board[i][j] == '.' {

                for &ch in row_choices[i].clone().intersection(&col_choices[j].clone()) {
                    let sqr_idx = 3 * (i / 3) + (j / 3);
                    if sqr_choices[sqr_idx].clone().contains(&ch) {
                        board[i][j] = ch;
                        row_choices[i].remove(&ch);
                        col_choices[j].remove(&ch);
                        sqr_choices[sqr_idx].remove(&ch);
                        let res = if j < 8 {
                            back_track(i, j+1, board, row_choices, col_choices, sqr_choices)
                        } else {
                            back_track(i+1, 0, board, row_choices, col_choices, sqr_choices)
                        };
                        if res { return true; }
                        else {
                            board[i][j] = '.';
                            row_choices[i].insert(ch);
                            col_choices[j].insert(ch);
                            sqr_choices[sqr_idx].insert(ch);
                        }
                    }
                }
                return false;
            } else {
                let res = if j < 8 {
                    back_track(i, j+1, board, row_choices, col_choices, sqr_choices)
                } else {
                    back_track(i+1, 0, board, row_choices, col_choices, sqr_choices)
                };

                return res;
            }
        }

        back_track(0, 0, board, &mut row_choices, &mut col_choices, &mut sqr_choices);
    }
}
// @lc code=end

