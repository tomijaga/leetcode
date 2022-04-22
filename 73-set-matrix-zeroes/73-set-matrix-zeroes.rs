pub fn reset_to_zero(matrix: &mut Vec<Vec<i32>>, x: usize, y: usize){
    let m = matrix.len();
    let n = matrix[0].len();
    
    for i in 0..m{
        matrix[i][y] = 0;
    }
    
    for j in 0..n{
        matrix[x][j] = 0;
    }
}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut zeroes = vec![];
        
        let m = matrix.len();
        let n = matrix[0].len();
        
        for i in 0..m{
            for j in 0..n{
                if matrix[i][j] == 0{
                    zeroes.push((i, j));
                }
            }
        }
        
        // println!("{:?}", zeroes);
        
        for (x, y) in zeroes.iter(){
            reset_to_zero(matrix, *x, *y);
        }
        
    }
}