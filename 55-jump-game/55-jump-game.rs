impl Solution {
    pub fn can_jump(mut nums: Vec<i32>) -> bool {
        let mut last_true_index = nums.len() - 1;
        
        for i in (0..nums.len() - 1).rev(){
            let mut res = false;
            
            let upper_bound = (nums.len() - 1 - i).min(nums[i] as usize);
            
            if i + upper_bound >= last_true_index{
                last_true_index = i
            }
        }
        
        last_true_index == 0
    }
}