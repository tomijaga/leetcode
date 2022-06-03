struct NumMatrix {
    matrix : Vec<Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        
        let mut new_matrix = vec![vec![0; n + 1]; m];
        
        for i in 0..m{
            let mut sum = 0;
            for j in 0..n{
                sum += matrix[i][j];
                new_matrix[i][j+1] = sum;
            }
        }
        
        Self{
            matrix: new_matrix
        }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        
        for i in row1..row2 + 1{
            sum += self.matrix[i][col2 + 1] - self.matrix[i][col1];
        }
        
        sum
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */