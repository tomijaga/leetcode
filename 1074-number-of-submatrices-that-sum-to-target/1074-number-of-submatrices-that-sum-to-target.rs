impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
	    // create prefix 2d matrix, first
        let m = matrix.len();
        let n = matrix[0].len();
        let mut prefix_matrix = vec![vec![0; n]; m];
        prefix_matrix[0][0] = matrix[0][0];
        for i in 1..m {
            prefix_matrix[i][0] = prefix_matrix[i - 1][0] + matrix[i][0];
        }
        for i in 1..n {
            prefix_matrix[0][i] = prefix_matrix[0][i - 1] + matrix[0][i];
        }
        for i in 1..m {
            for j in 1..n {
                prefix_matrix[i][j] = prefix_matrix[i - 1][j] + prefix_matrix[i][j - 1]
                    - prefix_matrix[i - 1][j - 1]
                    + matrix[i][j];
            }
        }

        let mut count = 0;
		
		// check submatrix sum is target  
		// query: start index (sx, sy) to end index (ex, ey)
        for sx in 0..m {
            for sy in 0..n {
                for ex in sx..m {
                    for ey in sy..n {
                        let mut res = prefix_matrix[ex][ey];
                        if sx > 0 {
                            res -= prefix_matrix[sx - 1][ey];
                        }
                        if sy > 0 {
                            res -= prefix_matrix[ex][sy - 1];
                        }
                        if sx > 0 && sy > 0 {
                            res += prefix_matrix[sx - 1][sy - 1];
                        }

                        if res == target {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}