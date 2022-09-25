use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize
}

impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        
        for n in nums{
            heap.push(Reverse(n));
            if heap.len() > k{
                heap.pop();
            }
        }
        
        Self{ k, heap }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        
        if self.heap.len() > self.k{
            self.heap.pop();
        }
        
        let Reverse(n) = self.heap.peek().unwrap();
        *n
        
    }
}