impl Solution {
    pub fn generate_matrix(len: i32) -> Vec<Vec<i32>> {
        let len = len as usize;
        let mut matrix = vec![vec![0_i32; len]; len];
            
        let mut n:i32 = 1_i32 ;
            
        let mut top = 0;
        let mut bottom = len;
        let mut left = 0;
        let mut right = len;
        
        while top < bottom && left < right{
            for j in left..right{
                matrix[top][j] = n;
                n+=1;
            }
            top +=1;
            
            if top < bottom && left < right{
                for i in top..bottom{
                    matrix[i][right-1] = n;
                    n+=1;
                }
                right -=1;
            }
            
            if top < bottom && left < right{
                for j in (left..right).rev(){
                    matrix[bottom-1][j] = n;
                    n+=1;
                }
                bottom -=1;
            }
            
            if top < bottom && left < right{
                for i in (top..bottom).rev(){
                    matrix[i][left] = n;
                    n+=1;
                }
                left +=1;
            }
        }
        
        matrix
    }
}