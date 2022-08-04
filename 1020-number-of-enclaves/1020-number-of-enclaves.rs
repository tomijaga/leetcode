pub const DIR : [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        for i in 0..m{
            for j in [0, n - 1]{
                dfs_mark_boundary(&mut grid, i, j);
            }
        }
        
        for j in 0..n{
            for i in [0, m - 1]{
                dfs_mark_boundary(&mut grid, i, j);
            }
        }
        
        let mut cnt = 0;
        
        for i in 1..m-1{
            for j in 1..n-1{
                if grid[i][j] == 1{
                    cnt += 1;
                }
            }
        }
        
        cnt
    }
}

pub fn dfs_mark_boundary(grid: &mut Vec<Vec<i32>>, i: usize, j: usize){
    let (m, n) = (grid.len(), grid[0].len());
    
    if grid[i][j] == 0{
        return;
    }
    
    // mark as visited
    grid[i][j] = 2;
    
    for (dx, dy) in DIR{
        let i = (dx + i as i32) as usize;
        let j = (dy + j as i32) as usize;
        
        if i!=usize::MAX && j!=usize::MAX && i < m && j < n && grid[i][j] == 1{
            dfs_mark_boundary(grid, i, j);
        }
    }
}