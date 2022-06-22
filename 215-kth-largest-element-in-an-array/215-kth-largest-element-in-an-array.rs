use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
//         let mut heap = BinaryHeap::from(nums);
        
//         for n in nums{
//             heap.push(Reverse(n));
//             if heap.len() > k as usize{
//                 heap.pop();
//             }
//         }
        
//         if let Some(Reverse(kth_largest)) = heap.pop(){
//             kth_largest
//         }else{
//             0
//         }
        
        nums.sort_unstable();
        nums[nums.len() - k as usize]
    }
}