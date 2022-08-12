impl Solution {
    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        let mut stack = vec![];

        stack.append(&mut rooms[0]);
        
        while !stack.is_empty(){
            let i = stack.pop().unwrap() as usize;
            
            if rooms[i].len() > 0{
                stack.append(&mut rooms[i]);
            }
        }
        
        rooms.into_iter().all(|v| v.is_empty())
    }
}