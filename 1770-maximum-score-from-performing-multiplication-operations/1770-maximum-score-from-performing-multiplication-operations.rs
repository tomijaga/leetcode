use std::collections::BTreeMap;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut dp = vec![vec![None; multipliers.len()]; multipliers.len()];
        
        backtrack(&nums, &multipliers, &mut dp, 0, 0)
    }
}

pub fn backtrack(
    nums: &[i32], 
    multipliers: &[i32], 
    dp: &mut Vec<Vec<Option<i32>>>, 
    i: usize, 
    op: usize
) -> i32{
    if multipliers.len() == op{
        return 0;
    }
    
    if let Some(score) = &dp[op][i]{
        return *score;
    }
    
    let j = nums.len() - 1 - (op - i);
    
    let m = multipliers[op];
    
    let n1 = nums[i];
    let n2 = nums[j];

    let top = (n1 * m) + backtrack(nums, multipliers, dp, i + 1, op + 1);
    let bottom = (n2 * m) +  backtrack(nums, multipliers, dp, i, op + 1);
    
    let score = top.max(bottom);
    
    dp[op][i] = Some(score);
    
    score
    
}