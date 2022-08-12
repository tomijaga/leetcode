use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        
        let (mut si, mut sj) = (0, 0);
        let mut islands = 0;
        
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == '1'{
                    grid[i][j] = '0';
                    bfs(&mut grid, i, j);
                    islands +=1;
                }
            }
        }
        
        islands
        
    }
}

fn bfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize){
    let (m, n) = (grid.len(), grid[0].len());
    let mut q = VecDeque::from([(i, j)]);
    
    while !q.is_empty(){
        let (i, j) = q.pop_front().unwrap();
        
        for (x, y) in [(i + 1, j), (i - 1, j), (i, j+ 1),(i, j-1)]{
            if x!=usize::MAX && x < m && y!=usize::MAX && y < n && grid[x][y] == '1'{
                q.push_back((x, y));
                grid[x][y] = '0';
            }
        }
    }
}