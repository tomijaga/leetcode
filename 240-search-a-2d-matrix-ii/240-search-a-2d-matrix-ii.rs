impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());

        let mut row = m - 1;
        let mut col = 0;
        
            while col < n && row >=0  {
                
                if matrix[row][col] < target{
                    col+= 1;
                }else if matrix[row][col] > target{
                    if row == 0{
                        return false;
                    }
                    row-=1;
                }else{
                    return true;
                }
            }
        
        false
        
    }
}