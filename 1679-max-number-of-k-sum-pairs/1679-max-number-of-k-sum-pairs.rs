impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        let mut count = 0;
        
        while left < right{
            let sum = nums[left] + nums[right];
            
            if sum < k{
                left +=1;
            }else if sum > k {
                right -=1;
            }else{
                left +=1;
                right-=1;
                count+=1;
            }
        }
        
        count
    }
}