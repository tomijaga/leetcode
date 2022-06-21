use std::collections::HashSet;

impl Solution {
    pub fn find_circle_num(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new();
        
        let n = grid.len();
        
        let mut cnt = 0;
        
        for i in 0..n{
            if !visited.contains(&i){
                if dfs(&mut grid, &mut visited, i){
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

pub fn dfs(grid: &mut Vec<Vec<i32>>, visited:&mut HashSet<usize>, i: usize) -> bool {
    let n = grid.len();
    
    if visited.contains(&i) {
        return false;
    }
    
    visited.insert(i);
    
    for j in 0..n{
        if grid[i][j] == 1 {
            dfs(grid, visited, j);
        }
    }
    // grid[i][j] = 3;

    return true;
}