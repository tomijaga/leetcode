use std::cmp::min;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        
        let mut memo = vec![];
        for i in 0..triangle.len(){
            memo.push(vec![i32::MAX; triangle[i].len()]);
        }
        
        dfs(&triangle, &mut memo, 0, 0)
    }
}

pub fn dfs(triangle: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32{
    if row >= triangle.len() || col >= triangle[row].len(){
        return i32::MAX;
    }else if memo[row][col] < i32::MAX{
        return memo[row][col];
    }
    
    let mut path_sum = triangle[row][col];
    
    if row < triangle.len() -1{
        path_sum += min( dfs(triangle, memo, row + 1, col), dfs(triangle, memo, row +1, col + 1));
    }
    
    memo[row][col] = path_sum;
    path_sum
}