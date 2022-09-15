impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![None; nums.len()];
        dp[nums.len() -1] = Some(0);
        let len = nums.len();
        
        for (i, n) in nums.into_iter().take(len - 1).enumerate().rev(){
            let upper_limit =  (i + n as usize).min(len - 1);
            
            let mut min_jumps = i32::MAX;
            for j in i..=upper_limit{
                // println!("--{:?}", (j, upper_limit, len));
                if let Some(jumps) = &dp[j]{
                    min_jumps = min_jumps.min(*jumps);
                }
            }
            
            if min_jumps != i32::MAX{
                dp[i] = Some(min_jumps + 1);
            }
            // println!("{:?}", (i, min_jumps));
            
        }
        
        // println!("{:?}", &dp);
        
        dp[0].unwrap()
    }
}