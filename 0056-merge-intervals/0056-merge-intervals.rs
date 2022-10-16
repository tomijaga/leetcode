impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        
        intervals.sort_unstable();
        
        let mut merged = vec![intervals[0].clone()];
        
        for interval in intervals.into_iter().skip(1){
            if let Some(last) = merged.last_mut(){
                if last[1] >= interval[0]{
                    last[1] = interval[1].max(last[1]);
                    last[0] = last[0].min(interval[0]);
                }else{
                    merged.push(interval);
                }
            }
        }
        
        merged
    }
}