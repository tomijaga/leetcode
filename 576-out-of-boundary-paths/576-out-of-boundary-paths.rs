use std::iter::successors;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let m = m as usize;
        let n = n as usize;
        let mut start = vec![vec![0; n]; m];
        start[start_row as usize][start_column as usize] = 1;
        (0..max_move)
            .scan(start, |cur, _| {
                let mut next = vec![vec![0; n]; m];
                let mut ans = 0;
                for (i, row) in cur.iter().enumerate() {
                    for (j, &x) in row.iter().enumerate() {
                        for (mut di, mut dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                            di += i as isize;
                            dj += j as isize;
                            if 0 <= di && 0 <= dj {
                                let di = di as usize;
                                let dj = dj as usize;
                                if di < m && dj < n {
                                    next[di][dj] = (next[di][dj] + x) % MOD;
                                    continue;
                                }
                            }

                            ans = (ans + x) % MOD;
                        }
                    }
                }
                *cur = next;
                Some(ans)
            })
            .fold(0, |x, y| (x + y) % MOD)
    }
}