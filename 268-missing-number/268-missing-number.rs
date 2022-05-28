impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        for i in 0..nums.len() -1{
            if nums[i + 1] - nums[i] > 1{
                return nums[i] + 1;
            }
        }
        
        if nums.first().unwrap() == &0{
            (*nums.last().unwrap()) + 1
        }else{
            0
        }
        
    }
}