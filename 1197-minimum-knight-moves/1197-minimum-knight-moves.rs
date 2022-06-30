use std::collections::{HashSet, VecDeque};

type Point = (i32,i32);

impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let (x, y) = (x.abs(), y.abs());
        let mut visited = HashSet::new();
        let mut q = VecDeque::from([(0, 0)]);
        
        let mut n = 0;
        
        while !q.is_empty(){
            
            for _ in 0..q.len(){
                
                let (i, j) = q.pop_front().unwrap();
            
                if (i, j) == (x, y){
                    return n;
                }
                
                if i>=-1 && j>=-1 && !visited.contains(&(i, j)){
        
                    visited.insert((i,j));
                    
                    let opts = [
                        (i + 2, j - 1),
                        (i + 2, j + 1),
                        (i - 2, j - 1),
                        (i - 2, j + 1),
                        (i + 1, j - 2),
                        (i + 1, j + 2),
                        (i - 1, j - 2),
                        (i - 1, j + 2)
                    ];

                    for (x,y) in opts{
                        q.push_back((x, y));
                    }
                    
                }
                
            }
            
            n+=1;
        }
        
        unreachable!()
    }
}
