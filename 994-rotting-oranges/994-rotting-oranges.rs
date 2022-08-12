use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        let (m, n) = (grid.len(), grid[0].len());
        
        let (empty, fresh, rotten) = (0, 1, 2);
        let mut fresh_oranges = 0;
        let mut mins = 0;
        
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == rotten{
                    q.push_back((i, j));
                }else if grid[i][j] == fresh{
                    fresh_oranges +=1;
                }
            }
        }
        
        
        while !q.is_empty(){
            let mut any_spoiled = false;
            
            for _ in 0..q.len(){
                let (i, j) = q.pop_front().unwrap();
                
                for (x, y) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j-1)]{
                    
                    if x!= usize::MAX && y!= usize::MAX && x < m && y < n && grid[x][y] == fresh{
                        q.push_back((x, y));
                        fresh_oranges -=1;
                        
                        grid[x][y] = rotten;
                        any_spoiled = true;
                    }
                }
            }
            
            if any_spoiled{
                mins +=1;
            }
        }
        
        if fresh_oranges == 0{
            mins
        }else{
            -1
        }
    }
}