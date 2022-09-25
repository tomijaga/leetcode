use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        
        for n in nums{
            heap.push(Reverse(n));
            
            if heap.len() > k{
                heap.pop();
            }
        }
        
        let Reverse(n) = heap.pop().unwrap();
        n
    }
}