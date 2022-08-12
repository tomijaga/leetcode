impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut area = 0;
        
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == 1{
                    area = area.max(dfs(&mut grid, i, j));
                }
            }
        }
        
        area
    }
}

fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32{
    let (m, n) = (grid.len(), grid[0].len());
    
    grid[i][j] = 0;
    
    let mut sum = 1;

    for (i, j) in [(i+1, j), (i-1, j), (i, j+1), (i, j-1)]{
        if i != usize::MAX && j != usize::MAX && i < m && j < n && grid[i][j] == 1{
            sum += dfs(grid, i, j);
        }
    }
    
    sum
}