use std::cmp::{max, min};

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        
        if intervals.len() <= 1{
            return intervals;
        }
        
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b|{ a[0].cmp(&b[0])});
        
        let mut result = vec![intervals[0].clone()];
        
        for i in 1..intervals.len(){
            let prev = result.last().unwrap();
            let next = &intervals[i];
            
            if prev[1] >= next[0] {
                let len = result.len();
                result[len - 1][1] = max(prev[1], next[1]);
            }else{
                result.push(intervals[i].clone());
            }
        }
        result
    }
}