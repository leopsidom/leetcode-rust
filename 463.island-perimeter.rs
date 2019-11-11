/*
 * @lc app=leetcode id=463 lang=rust
 *
 * [463] Island Perimeter
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let R = grid.len();
        if R == 0 { return 0; }
        let C = grid[0].len();
        let mut seen = HashSet::new();
        let mut ans = 0;

        fn dfs(
            coord: (usize, usize), 
            seen: &mut HashSet<(usize, usize)>, 
            R: usize, C: usize, 
            ans: &mut u32,
            grid: &Vec<Vec<i32>>
        ) {
            seen.insert(coord);
            // println!("{:?}", coord);
            // *n_nodes += 1;
            let (x, y) = coord;

            let mut neis = vec![];
            if x > 0 {
                neis.push((x-1, y));
            } else { *ans += 1; }

            if x < R - 1 {
                neis.push((x+1, y));
            } else { *ans += 1; }

            if y > 0 {
                neis.push((x, y-1));
            } else { *ans += 1; }

            if y < C - 1 {
                neis.push((x, y+1));
            } else { *ans += 1; }

            for (i, j) in neis {
                // println!("seen {:?} contains {:?}? {}", seen, (i, j), seen.contains(&(i, j)));
                if grid[i][j] == 1 {
                    if !seen.contains(&(i, j)) {
                        dfs((i, j), seen, R, C, ans, grid);
                    }
                } else {
                    *ans += 1;
                }
            }
        }

        for i in 0..R {
            for j in 0..C {
                if grid[i][j] == 1 {
                    if !seen.contains(&(i, j)) {
                        dfs((i, j), &mut seen, R, C, &mut ans, &grid);
                    }
                }
            }
        }

        // println!("{}, {}", n_nodes, n_edges);
        ans as i32
    }
}
// @lc code=end

