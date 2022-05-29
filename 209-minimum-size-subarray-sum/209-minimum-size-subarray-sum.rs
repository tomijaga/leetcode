use std::cmp::min;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        
        let mut nums = nums;
        
        nums.push(0);
        
        let mut n = nums[0];
        nums[0] = 0;
        for i in 1..nums.len(){
            let prev = nums[i];
            nums[i] = nums[i-1] + n;
            n = prev;
        }
        
        
        
        for i in 0..nums.len(){
            let mut left = i + 1;
            let mut right = nums.len();
            
            while (left < right){
                let mid = left + (right - left ) /2;
                
                let sum = nums[mid] - nums[i];
                
                if sum >= target{
                    right = mid;
                    ans = min((mid - i) as i32, ans);
                    
                }else{
                    left = mid+1;
                }
            }
        }
        
        if ans == i32::MAX {
            0
        }else{
            ans
        }
    }
}