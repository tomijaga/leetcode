use std::{
    collections::BinaryHeap,
    cmp::{Ordering}
};

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by(|a, b|{a[1].cmp(&b[1])});
        
        let mut heap = BinaryHeap::new();
        let mut days_elapsed = 0;
        let mut cnt = 0;

        for v in courses{
            
            if v[0] <=  v[1]{
                days_elapsed += v[0];
                heap.push(v[0]);
                
                if days_elapsed > v[1]{
                    if let Some(top) = heap.pop(){
                        days_elapsed -= top;
                    }
                }
            }
            
        }
        
        heap.len() as i32
    }
}
