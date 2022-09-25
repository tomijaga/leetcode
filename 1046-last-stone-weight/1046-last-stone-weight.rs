use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into();
        
        while heap.len() >= 2{
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            
            let diff = y - x;
            if diff > 0{
                heap.push(diff);
            }
        }
        
        if heap.len()== 0{
            0
        }else{
            heap.pop().unwrap()
        }
    }
}