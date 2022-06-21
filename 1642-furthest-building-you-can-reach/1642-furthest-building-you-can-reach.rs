use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn furthest_building(mut heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        for i in (1..heights.len()).rev(){
            heights[i] = heights[i] - heights[i - 1];
        }
        
        heights[0] = 0;
        
        let diffs = heights;
        
        // println!("diffs: {:?}", &diffs);
        
        let mut heap = BinaryHeap::new();
        
        let len = diffs.len();
        
        for (i, diff) in diffs.into_iter().enumerate(){
            if diff > 0{
                heap.push(Reverse(diff));
            }
            
            if heap.len() > ladders as usize{
                let Reverse(least_diff) = heap.pop().unwrap();
                bricks -= least_diff;
                if bricks < 0{
                    // println!("l: {:?}", (&heap.len(), bricks));
                    return (i - 1) as i32;
                }
            }
            // println!("{:?}", (&heap, bricks));
            
        }
        
        (len - 1) as i32
    }
}