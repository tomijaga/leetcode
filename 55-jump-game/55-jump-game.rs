impl Solution {
    pub fn can_jump(mut nums: Vec<i32>) -> bool {
        let mut dp = vec![None; nums.len()];
        let last_ptr = dp.last_mut().unwrap();
        *last_ptr = Some(true);
        
        backtrack(&nums, &mut dp, 0)
    }
}

fn backtrack(nums: &Vec<i32>, dp: &mut Vec<Option<bool>>, i: usize) -> bool {
    if i == nums.len() - 1{
        return true;
    }
    
    let upper_bound = (nums.len() - 1 - i).min(nums[i] as usize);
    
    for di in (1..=upper_bound){
        let j = i + di;
        
        if let Some(val) = &dp[j]{
            if *val{
                return true;
            }
        }else if backtrack(nums, dp, j){
            dp[j] = Some(true); 
            return true;
        }else{
            dp[j] = Some(false);
        }
    }
    
    false
}