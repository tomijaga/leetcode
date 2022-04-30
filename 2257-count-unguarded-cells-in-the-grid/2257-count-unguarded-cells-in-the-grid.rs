use std::cmp::max;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        //  cell indicators:
        //      0 - not guarded
        //      1 - guarded
        //      7 - guard
        //      10 - wall
        
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
        
        for wall in walls{
            let i = wall[0] as usize;
            let j = wall[1]  as usize;
            
            matrix[i][j] = 10;
        }
        
        for guard in guards.iter(){
            let i = guard[0]  as usize;
            let j = guard[1]  as usize;
            
            matrix[i][j] = 7;
        }
        
        for guard in guards{
            let i = guard[0]  as usize;
            let j = guard[1]  as usize;
            
            guard_cells(&mut matrix, i, j);
        }
        
        let mut n = 0;
        
        for i in 0..matrix.len(){
            for j in 0..matrix[0].len(){
                if matrix[i][j] == 0{
                    n+=1;
                }
            }
        }
        
        n
    }
}

pub fn guard_cells(matrix: &mut Vec<Vec<i32>>, i: usize, j: usize){
    
    fn guard_cell(matrix: &mut Vec<Vec<i32>>, i: usize, j:usize)-> bool{
        if(i < matrix.len() && j < matrix[0].len() && i >= 0 && j >= 0){
            if (matrix[i][j] == 10 || matrix[i][j] == 7){
                return false;
            }else {
                matrix[i][j] = 1;
                return true;
            }
        }
        
        return false;
    }
    
    let mut dir = [true;4]; // [↑, ↓, <-, ->]
    
    for diff in 1..max(matrix.len(), matrix[0].len()){
        if dir[0]{
            if !guard_cell(matrix, i-diff, j){
                dir[0] = false;
            }
        }
        
        if dir[1]{
            if !guard_cell(matrix, i+diff, j){
                dir[1] = false;
            }
        }
        
        if dir[2]{
            if !guard_cell(matrix, i, j - diff){
                dir[2] = false;
            }
        }
        
        if dir[3]{
            if !guard_cell(matrix, i, j + diff){
                dir[3] = false;
            }
        }
        
        if dir == [false;4]{
            break;
        }
    }
    
}