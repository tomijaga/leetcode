pub type Point = (usize, usize);

pub fn get_next(limits: Point, curr: Point) -> Point{
    let (m, n) = limits;
    let (mut i, mut j) = curr;
    if (j+1>=n){
        j = 0;
        i+=1;
    }else{
        j+=1;
    }
    
    (i, j)
}
    
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        
        let (m, n) = (mat.len(), mat[0].len());
        
        if r * c != m * n {
            return mat;
        }
        
        let mut new_mat = vec![vec![0; c]; r];
        
        let (mut x, mut y) = (0, 0);
        
        for i in 0..m{
            for j in 0..n{
                let val = mat[i][j];
                
                new_mat[x][y] = val;
                
                let point = get_next((r, c), (x, y));
                // println!("{:?}", point);
                x = point.0;
                y = point.1;
            }
        }
        
        new_mat
    }
}