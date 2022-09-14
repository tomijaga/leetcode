impl Solution {
    pub fn can_jump(mut nums: Vec<i32>) -> bool {
        let mut dp = vec![None; nums.len()];
        let last_ptr = dp.last_mut().unwrap();
        *last_ptr = Some(true);
        
        for i in (0..nums.len() - 1).rev(){
            let mut res = false;
            
            let upper_bound = (nums.len() - 1 - i).min(nums[i] as usize);
            
            for di in 1..=upper_bound{
                let j = i + di;
                
                if &Some(true) == &dp[j]{
                    res = true; 
                    break;
                }
            }
            
            dp[i] = Some(res);
            
        }
        
        &Some(true) == &dp[0]
    }
}