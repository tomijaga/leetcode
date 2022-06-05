impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());

        for i in 0..m{
            let mut l = 0;
            let mut r = n;
            
            while l < r{
                let mid = l + (r - l)/2;
                
                if matrix[i][mid] < target{
                    l= mid + 1;
                }else if matrix[i][mid] > target{
                    r = mid;
                }else{
                    return true;
                }
            }
        }
        
        false
        
    }
}