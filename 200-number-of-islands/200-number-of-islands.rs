use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if (grid.len() == 0){
            return 0;
        }
        
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let (row, col) = (grid.len(), grid[0].len());
        
        let mut n = 0;
        
        for i in 0..row{
            for j in 0..col{
                let mut queue = VecDeque::from([(i, j)]);
                
                let mut result = false;
                while let Some((_i , _j)) = queue.pop_front() {
                    result |= bfs(&grid, &mut visited, &mut queue, _i, _j);
                }
                
                // println!("(i: {:?}, j: {:?}) result: {:?}", _i, )
                
                if result {
                    n+=1;
                }
            }
        }
        
        n
    }
}

pub fn bfs(grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>, queue: &mut VecDeque<(usize, usize)>, i: usize, j: usize) -> bool {
    let (row, col) = (grid.len(), grid[0].len());
    
    if (!(0..row).contains(&i) || !(0..col).contains(&j) || visited.contains(&(i, j)) || grid[i][j] == '0'){
        return false;
    }
    
    queue.push_back((i+1, j));
    queue.push_back((i-1, j));
    queue.push_back((i, j-1));
    queue.push_back((i, j+1));
    
    visited.insert((i, j));
    
    return true;
}