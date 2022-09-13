impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr_sum = 0;
        let mut max_sum = i32::MIN;
        
        for n in nums{
            curr_sum += n;
            
            if curr_sum > max_sum {
                max_sum = curr_sum;
            }
            
            if curr_sum < 0{
                curr_sum = 0;
            }
        }
        
        max_sum
    }
}