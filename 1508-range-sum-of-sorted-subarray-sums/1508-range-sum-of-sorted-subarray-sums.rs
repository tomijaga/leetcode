use std::{
    cmp::Reverse,
    collections::BinaryHeap
};

impl Solution {
    pub fn range_sum(mut nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        
        let mut prev = nums[0];
        nums[0] = 0;
        for i in 1..nums.len(){
            let tmp = prev;
            prev = nums[i];
            
            nums[i] = nums[i - 1] + tmp;
        }
        
        nums.push(prev + *nums.last().unwrap());
        
        let mut heap = BinaryHeap::new();
        let mut lower_heap = BinaryHeap::new();
        
        let mut sum: i64 = 0;
        for i in 0..nums.len(){
            for j in i+1..nums.len(){
                let n = nums[j] - nums[i];
                
                lower_heap.push(n);
                
                if lower_heap.len() >= left {
                    let n = lower_heap.pop().unwrap();
                    
                    heap.push(n);
                    sum+=n as i64;
                }
                
                if heap.len() > right - left + 1{
                    sum -= heap.pop().unwrap() as i64;
                }
            }
        }
        
        // println!("{:?}", &lower_heap); 
        // println!("{:?}", &heap); 
        
        (sum % (10_i64.pow(9) + 7)) as i32
    }
}