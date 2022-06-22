use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        
        for n in nums{
            heap.push(Reverse(n));
            if heap.len() > k as usize{
                heap.pop();
            }
        }
        
        if let Some(Reverse(kth_largest)) = heap.pop(){
            kth_largest
        }else{
            0
        }
        
    }
}