impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1{
            return nums[0];
        }
        
        let mut dp = vec![nums[0], nums[0].max(nums[1])];
        
        for i in 2..nums.len(){
            dp.push( dp[i-1].max(dp[i-2] + nums[i]) );
        }
        
        dp[nums.len() - 1]
    }
}