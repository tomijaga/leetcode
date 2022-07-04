//  matrix cell indicators:
        //      0 - not guarded
        //      1 - guarded
        //      2 - guard
        //      3 - wall

use std::cmp::max;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        
        let (UNGUARDED, GUARDED, GUARD, WALL) = (0, 1, 2, 3);
        
        let mut cnt = (m * n) - guards.len() as i32 - walls.len() as i32;
        
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
        
        for wall in walls{
            let i = wall[0] as usize;
            let j = wall[1]  as usize;
            
            matrix[i][j] = WALL;
        }
        
        for guard in guards.iter(){
            let i = guard[0]  as usize;
            let j = guard[1]  as usize;
            
            matrix[i][j] = GUARD;
        }
        
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        
        // For every guard change all unguarded cells 
        // in all four cardinal directions from it to
        // guarded cells.
        // Stop this process if it encounters another 
        // guard or a wall
        
        for guard in guards{
            for &(dx, dy) in dirs.iter(){
                let mut x = guard[0] + dx;
                let mut y = guard[1] + dy; 
                
                while x >= 0 && y >= 0 && x<m && y<n {
                    let (i,j) = (x as usize, y as usize);
                    
                    if matrix[i][j] == WALL || matrix[i][j] == GUARD{
                        break;
                    }
                    
                    if matrix[i][j] == UNGUARDED{
                        cnt -=1;
                    }
                    matrix[i][j] = GUARDED;
                    x += dx;
                    y += dy;
                } 
            }
            
        }
        
        cnt
    }
}