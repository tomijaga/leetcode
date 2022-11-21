use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let (CELL, WALL) = ('.', '+');
        let (m, n) = (maze.len(), maze[0].len());
        
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let mut q = VecDeque::from([entrance]);
        maze[entrance.0][entrance.1] = WALL;
        
        let mut dist = 0;
        while !q.is_empty(){
            for _ in 0..q.len(){
                let (i, j) = q.pop_back().unwrap();
                
                for (x, y) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]{
                    if x < m && x!= usize::MAX && y < n && y!= usize::MAX && maze[x][y] == CELL{
                        if x == 0 || x == m - 1 || y == 0 || y == n - 1{
                            return dist + 1;
                        }
                        maze[x][y] = WALL;
                        q.push_front((x, y));
                    }
                }
            }
            
            dist+= 1;
        }
        
        -1
    }
}