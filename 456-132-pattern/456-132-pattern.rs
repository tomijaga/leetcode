use std::cmp;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut n3 =  i32::MIN;
        
        for i in (0..nums.len()).rev(){
            if nums[i] < n3{
                return true;
            }else {
                while !stack.is_empty() && nums[i] > *stack.last().unwrap(){
                    n3 = stack.pop().unwrap();
                }
                
                stack.push(nums[i]);
            }
        }
        
        false
    }
}