use std::collections::{ VecDeque };

impl Solution {
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::<(usize, usize, i32)>::new();
        let mut max_d = 0;
        
        let (m, n) = (grid.len(), grid[0].len());
        
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == 1{
                    grid[i][j] = 2; // mark as visited
                    q.push_back((i, j, 0));
                }
            }
        }
        
        while !q.is_empty(){
        
            for _ in 0..q.len(){
                let (i, j, dist) = q.pop_front().unwrap();
                max_d = max_d.max(dist);
                
                for (x, y) in [(i+1, j), (i, j+1), (i-1, j), (i, j-1)] {
                    if (x!= usize::MAX && x < m && y!= usize::MAX && y < n && grid[x][y] <= 1){
                        grid[x][y] = 2; // mark as visited
                        q.push_back((x, y, dist + 1));
                    }
                }
            }
        }
        
        // println!("{:?}", &grid);
        
        if max_d == 0 { return -1 };
        
        max_d
    }
}