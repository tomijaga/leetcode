impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        
        let mut grid = vec![vec![0; n]; m];
        
        grid[0][0] = 1;
        
        for i in 0..m{
            for j in 0..n{
                let x = i - 1;
                let y = j - 1;
                
                if x != usize::MAX && x < m {
                    grid[i][j] += grid[x][j];  
                };
                
                if y != usize::MAX && y < n {
                    grid[i][j] += grid[i][y];  
                };
            }
        }
        
        grid[m - 1][n - 1]
    }
}