use std::collections::BTreeMap;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut dp = vec![BTreeMap::new(); multipliers.len()];
        
        backtrack(&nums, &multipliers, &mut dp, 0, nums.len() - 1, 0)
    }
}

pub fn backtrack(
    nums: &[i32], 
    multipliers: &[i32], 
    dp: &mut Vec<BTreeMap<(usize, usize), i32>>, 
    i: usize, 
    j: usize,
    op: usize
) -> i32{
    if multipliers.len() == op{
        return 0;
    }
    
    if let Some(&score) = dp[op].get(&(i, j)){
        return score;
    }
    
    let m = multipliers[op];
    
    let n1 = nums[i];
    let n2 = nums[j];

    let top = (n1 * m) + backtrack(nums, multipliers, dp, i + 1, j, op + 1);
    let bottom = (n2 * m) +  backtrack(nums, multipliers, dp, i, j - 1, op + 1);
    
    let score = top.max(bottom);
    
    dp[op].insert((i, j), score);
    
    score
    
}