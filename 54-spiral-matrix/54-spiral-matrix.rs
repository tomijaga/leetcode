pub type Point<T> = (T, T);

pub fn is_valid(top: usize, bottom: usize, left: usize, right: usize)->bool{
    top < bottom && left < right
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut spiral = vec![];
        
        let mut top = 0;
        let mut right = matrix[0].len();
        let mut left = 0;
        let mut bottom = matrix.len();
        
        while is_valid(top, bottom, left, right) {
            for j in left..right{
                spiral.push(matrix[top][j]);
            }
            top+=1;
            
        
            if is_valid(top, bottom, left, right){
            
                for i in top..bottom{
                    spiral.push(matrix[i][right -1]);
                }
                right-=1;
            }


            if is_valid(top, bottom, left, right){

                for j in (left..right).rev(){
                    spiral.push(matrix[bottom -1][j]);
                }
                bottom-=1;
            }
            
            if is_valid(top, bottom, left, right){
                for i in (top..bottom).rev(){
                    spiral.push(matrix[i][left]);
                }
                left+=1;
            }

        }
        spiral
    }
}