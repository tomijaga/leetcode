use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (usize, usize);

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new();
        
        let (r, c) = (grid.len(), grid[0].len());
        
        let mut max = 0;
        
        for i in 0..r{
            for j in 0..c{
                let mut queue: VecDeque<Point> = VecDeque::from([(i, j)]);
                let mut result = 0;
                
                while let Some(p) = queue.pop_front(){
                    if bfs(&grid, &mut visited, &mut queue, p){
                        result+=1;
                    }
                }
                
                if result > max{
                    max = result;
                }
            }
        }
        
        max
        
    }
}

pub fn bfs(grid: &Vec<Vec<i32>>, visited: &mut HashSet<Point>, queue: &mut VecDeque<Point>, p: Point)-> bool {
    let (i, j) = p;
    let (r, c) = (grid.len(), grid[0].len());
    
    
    if (!(0..r).contains(&i) || !(0..c).contains(&j) || visited.contains(&(p)) || grid[i][j] == 0){
        return false;
    }
    
    visited.insert(p);
    
    queue.push_back((i + 1, j));
    queue.push_back((i - 1, j));
    queue.push_back((i, j + 1));
    queue.push_back((i, j - 1));
    
    true
}