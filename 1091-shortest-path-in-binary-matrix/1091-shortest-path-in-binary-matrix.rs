use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        if grid[0][0] != 0{
            return -1;
        }
        
        let mut q = VecDeque::from([(0,0)]);
        let mut path = 0;
        
        while !q.is_empty(){
            let mut path_exists = false;
            let len = q.len();
            for _ in 0..len{
                
                let p = q.pop_front().unwrap();
                
                path_exists |= bfs(&mut grid, &mut q, p.0, p.1);
            }
            
            if path_exists{
                path+=1;
            }
            
            if grid[m- 1][n - 1] == 3{
                return path;
            }
        }
        
        // if the last grid was never visited
        // return -1
        if grid[m- 1][n - 1] != 3  {
            -1
        }else{
            path
        }
    }
}

fn bfs(grid: &mut Vec<Vec<i32>>, q: &mut VecDeque<(usize, usize)>, i: usize, j: usize) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    
    if i != usize::MAX && i < m && j != usize::MAX && j<n && grid[i][j] == 0{
        // mark as visited
        grid[i][j] = 3;
        
        q.push_back((i - 1, j));
        q.push_back((i + 1, j ));
        q.push_back((i, j + 1));
        q.push_back((i, j - 1));
        q.push_back((i - 1, j - 1));
        q.push_back((i - 1, j + 1));
        q.push_back((i + 1, j - 1));
        q.push_back((i + 1, j + 1));
         
        true
    }else{
        false
    }
}