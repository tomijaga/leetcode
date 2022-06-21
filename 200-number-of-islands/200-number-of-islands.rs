impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        
        let mut n = 0;
        
        for i in 0..row{
            for j in 0..col{
                
                let mut result = false;

                if grid[i][j] == '1'{
                    result |= dfs(&mut grid, i, j);
                    if result {
                        n+=1;
                    }
                }
                
            }
        }
        
        n
    }
}

pub fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let (row, col) = (grid.len(), grid[0].len());
    
    if (!(0..row).contains(&i) || !(0..col).contains(&j) ||  grid[i][j] != '1') {
        return false;
    }

    grid[i][j] = '#';
    
    dfs(grid, i+1, j);
    dfs(grid, i-1, j);
    dfs(grid, i, j-1);
    dfs(grid, i, j+1);
    
    return true;
}