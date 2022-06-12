use std::{
    cmp::max,
    collections::HashMap
};

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        
        let mut sum = 0_i64;
        
        let mut prefix_sum = vec![0; nums.len()+ 1];
        
        for i in 0..nums.len(){
            prefix_sum[i + 1] = nums[i]  as i64 + prefix_sum[i];
        }
        
        let mut i = 0;
        let mut len = 0;
        
        let mut max_sub = 0;
        for j in 0..nums.len(){
            if let Some(&n) = map.get(&nums[j]){
                while i <= n{
                    i += 1;
                }
            }
            
            // println!("{:?}", (i, j));
            
            map.insert(nums[j], j);
            max_sub = max(prefix_sum[j+1] - prefix_sum[i] as i64, max_sub);
            // println!("{:?}", &max_sub);
        }
        
        max_sub as i32
    }
}