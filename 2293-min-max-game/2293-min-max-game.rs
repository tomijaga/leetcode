use std::cmp::{min, max};

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        
        let mut n = nums.len()/2;
        
        while n > 0{
            for i in 0..n{
                if i % 2 ==0{
                    nums[i] = min(nums[2 * i], nums[2 * i + 1]);
                }else{
                    nums[i] = max(nums[2 * i], nums[2 * i + 1]);
                }
            }
            n /= 2;
        }
        
        nums[0]
    }
}