impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2{
            return nums.len() as i32;
        }
        
        let mut max = 1;
        let mut s = 0;
        
        for i in 1..nums.len(){
            if nums[i] < nums[i - 1] && s != -1{
                s = -1;
                max+=1;
            }else if nums[i] > nums[i-1] && s != 1{
                s = 1;
                max += 1;
            }
        }
        
        max
    }
}