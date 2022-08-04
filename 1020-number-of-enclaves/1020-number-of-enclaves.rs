pub const DIR : [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Solution {
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        let mut cnt = 0;
        for i in 1..m-1{
            for j in 1..n-1{
                if grid[i][j] == 1{
                    cnt += dfs(&mut grid, i, j);
                }
            }
        }
        
        cnt
    }
}

pub fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    
    if i == 0 || j == 0 || i == m - 1 || j == n-1{
        grid[i][j] = 0;
        return 0;
    }
    
    let mut cnt = if grid[i][j] == 1{
        1
    }else{
        0
    };
    
    let mut foundBoundaryNode = false;
    grid[i][j] = 2;
    
    for (dx, dy) in DIR{
        let x = (dx + i as i32) as usize;
        let y = (dy + j as i32) as usize;
        
        if grid[x][y] == 1{
            let res = dfs(grid, x, y);
            if res == 0{
                foundBoundaryNode = true;
            }
            
            cnt += res;
        }
    }
    
    if foundBoundaryNode{
        0
    }else{
        cnt
    }
}