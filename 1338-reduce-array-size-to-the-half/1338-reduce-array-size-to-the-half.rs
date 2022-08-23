use std::collections::{BTreeMap, BinaryHeap};

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut counter = vec![0; 100_000];
        let len = arr.len();
        
        for n in arr{
            counter[(n as usize) -1] +=1;
        }
        
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        
        for cnt in counter{
            
            sum += cnt;
            heap.push(-cnt);
            
            while (sum + *heap.peek().unwrap()) >= (len/2) as i32{
                sum+=heap.pop().unwrap();
            }
            
        }
        
        heap.len() as i32
    }
}