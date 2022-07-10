use std::collections::BinaryHeap;

impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut cache = nums.clone();
        let mut heap = BinaryHeap::from([(nums[0], 0)]);
        
        for i in 1..nums.len(){
            let n = nums[i];
            let mut peek = *heap.peek().unwrap();

            while (peek.1 as i32) < (i as i32 - k) {
                heap.pop();
                peek = *heap.peek().unwrap();
            }
            
            cache[i] += cache[heap.peek().unwrap().1];
            heap.push((cache[i], i));
            
            // println!("{:?}", cache);
        }
        
        *cache.last().unwrap()
    }
}
