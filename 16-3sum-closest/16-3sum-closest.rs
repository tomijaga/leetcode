use std::{i32, cmp};

pub fn diff(b: i32, a: i32)->i32{
    (b - a).abs()
}

pub fn is_closer(prev: i32, curr: i32, target: i32)->bool{
    
   diff(target, curr) < diff(target, prev)
}


impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        
        let mut closest = nums[0]+ nums[1]+nums[2];
        
        for i in 0..nums.len() -1{
            let mut j = i + 1;
            let mut k = nums.len() -1;
            
            while(j < k){
                let sum = nums[i]+ nums[j] + nums[k];
                // println!("{}", (target-closest ).abs());
                if (sum > target){
                    k-=1;
                }else if (sum < target){
                    j+=1;
                }else{
                    return sum;
                }
                
                if(is_closer(closest, sum, target)){
                    closest = sum;
                }
            }
        }
        
        closest
    }
}