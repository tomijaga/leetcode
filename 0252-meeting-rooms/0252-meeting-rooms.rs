impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable();
        let mut iter = intervals.into_iter();
        
        if let Some(mut prev) = iter.next(){
            for curr in iter{
                if prev[1] > curr[0]{
                    return false;
                }

                prev = curr;
            }
        }
        
        true
    }
}