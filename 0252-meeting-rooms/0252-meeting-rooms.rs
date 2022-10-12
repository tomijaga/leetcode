impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() == 0{
            return true;
        }
        
        intervals.sort_unstable();
      
        let mut iter = intervals.into_iter();
        
        let mut prev = iter.next().unwrap();
        
        for curr in iter{
            if prev[1] > curr[0]{
                return false;
            }
            
            prev = curr;
        }
        
        true
    }
}