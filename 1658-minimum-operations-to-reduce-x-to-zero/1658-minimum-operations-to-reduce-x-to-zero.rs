use std::{cmp, collections::HashMap};

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;
        
        if target == 0{
            return nums.len() as i32;
        }
        
        let mut map = HashMap::new();
        map.insert(0, -1);
        
        let mut sub_len = i32::MIN;
        
        let mut sum = 0;
        for (i, n) in nums.iter().enumerate(){
            sum += n;
            let i = i as i32;
            if let Some(&prefix_sum_len) = map.get(&(sum - target)){
                sub_len = cmp::max(sub_len, i - prefix_sum_len);
            }
            
            map.insert(sum, i);
        }
        
        if sub_len == i32::MIN{
            -1
        }else{
            nums.len() as i32 - sub_len
        }
    }
}