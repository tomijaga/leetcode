use std::collections::{VecDeque, HashMap, HashSet};

impl Solution {
    pub fn shortest_path(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut q = VecDeque::new();
        let mut set = HashSet::new();
        
        let (m, n) = (grid.len(), grid[0].len());
        
        q.push_back((0, 0, k));
        set.insert((0, 0, k));
        let mut dist = 0;
        let mut found_exit = false;
        
        // println!("{:?}", &q);
        
        while !q.is_empty(){
            for _ in 0..q.len(){
                
                let (i, j, skips) = q.pop_front().unwrap();
                if (i, j) == (m - 1, n - 1){
                    found_exit = true;
                    return dist;
                }
                
                for (x, y) in [(i + 1, j), (i, j + 1), (i, j-1), (i - 1, j)]{
                    if x != usize::MAX && x < m && y != usize::MAX && y < n{
                        let mut curr = (x, y, skips);
                        if grid[x][y] == 0{
                            if !set.contains(&curr){
                                q.push_back(curr);
                                set.insert(curr);
                            }
                            
                        }else if grid[x][y] == 1{
                            if skips > 0{
                                curr.2 -= 1;
                                if !set.contains(&curr){
                                    q.push_back(curr);
                                    set.insert(curr);
                                }
                            }
                        }
                    }
                }
            }
            
            // println!("{:?}", &q);
            
            dist+= 1;
        }
        
        - 1
    }
}