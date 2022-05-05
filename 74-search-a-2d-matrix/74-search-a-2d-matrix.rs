pub fn get_num(matrix: &Vec<Vec<i32>>, index: usize)-> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    
    
    let (i, j) = (index /n, index%n);
    let n = matrix[i][j];
    println!("{} -> ({}, {}) -> {}", index, i, j, n);
    n
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        
        let mut left = 0;
        let mut right = m*n;
        
        while left < right{
            let mid = left + (right - left)/2;
            let n = get_num(&matrix, mid);
            println!("left: {}, right:{}", left, right);
            
            if n < target{
                left = mid + 1;
            }else if n> target{
                right = mid;
            }else{
                return true;
            }
            
        }
        
        false
    }
}