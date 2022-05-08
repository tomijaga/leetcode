impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        
        // flip diagonally
        for i in 0..len{
            for j in (i +1)..len{
                swap(matrix, (i, j), (j, i));
            }
        }
         
        // flip vertically
        let mid  = len/2;
        for i in 0..len{
            for j in 0..mid{
                swap(matrix, (i, j), (i, len - j - 1));
            }
        }
    }
}

pub type Point = (usize, usize);

pub fn swap(matrix: &mut Vec<Vec<i32>>, a: Point, b: Point) {
    let (ai, aj) = a;
    let (bi, bj) = b;
    
    let temp = matrix[bi][bj];
    matrix[bi][bj] = matrix[ai][aj];
    matrix[ai][aj] = temp;
}