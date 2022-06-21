impl Solution {
    pub fn find_circle_num(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut cnt = 0;
        
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == 1{
                    if dfs(&mut grid, i, j){
                        cnt += 1;
                    }
                }
            }
        }
        
        cnt
    }
}

pub fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    
    if i == usize::MAX || i >= m || j == usize::MAX || j >= n || grid[i][j] !=1{
        return false;
    }
    
    grid[i][j] = 3;
    
    for j in 0..n{
        dfs(grid, i, j);
    }
    
    for i in 0..m{
        dfs(grid, i, j);
    }
    
    return true;
}