impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable();
        
        for w in intervals.windows(2){
            let a = &w[0];
            let b = &w[1];
            
            if a[1] > b[0]{
                return false;
            }
        }
        
        true
    }
}