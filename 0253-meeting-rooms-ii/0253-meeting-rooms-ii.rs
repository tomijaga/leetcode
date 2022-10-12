impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        
        let mut meeting_rooms = vec![intervals[0].clone()];
        
        
        for interval in intervals.into_iter().skip(1){
            let mut found_room = false;
            for room in meeting_rooms.iter_mut(){
                if room[1] > interval[0]{
                    continue;
                } else{
                    found_room = true;
                    *room = interval.clone();
                    break;
                }
            }
            
            if !found_room{
                meeting_rooms.push(interval);
            }
        }
        // println!("{:?}", &meeting_rooms); 
        meeting_rooms.len() as i32
    }
}