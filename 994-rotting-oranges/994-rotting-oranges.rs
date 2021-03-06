use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut rotten = VecDeque::new();
        let (m,n) = (grid.len(), grid[0].len());
        
        // println!("{:?}", (m, n));
        let mut fresh_oranges = 0;
        
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == 2{
                    rotten.push_back((i, j));
                }else if grid[i][j] ==1{
                    fresh_oranges+=1;
                }
            }
        }
        
        let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut days = 0;
        
        while rotten.len() > 0{
            let mut decayed = false;
            for _ in 0..rotten.len(){
                let (i, j) = rotten.pop_front().unwrap();
                
                for (dx, dy) in dirs.iter(){
                    let x  = i as i32 + dx;
                    let y  = j as i32 + dy;
                    
                    if x >= 0 && y>=0 && x < m as i32 && y < n as i32{
                        let (x, y) = (x as usize, y as usize);
                        
                        if grid[x][y] == 1{
                            grid[x][y] = 2;
                            fresh_oranges -=1;
                            rotten.push_back((x, y));
                            decayed = true;
                        }
                    }
                }
            }
            
            if decayed{
                days+=1;
            }
        }
        
        if fresh_oranges > 0{
            -1
        }else{
            days
        }
        
    }
}