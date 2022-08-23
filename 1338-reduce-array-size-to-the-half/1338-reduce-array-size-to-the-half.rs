use std::collections::{BTreeMap, BinaryHeap};

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut freq = BTreeMap::new();
        let len = arr.len();
        
        for n in arr{
            *freq.entry(n).or_insert(0_i32) +=1;
        }
        
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        
        for cnt in freq.into_values(){
            // println!(": {:?}", cnt);
            // println!("h {:?}", &heap);
            
            
            sum += cnt;
            heap.push(-cnt);
            
            while (sum + *heap.peek().unwrap()) >= (len/2) as i32{
                sum+=heap.pop().unwrap();
            }
            
        }
        
        heap.len() as i32
    }
}