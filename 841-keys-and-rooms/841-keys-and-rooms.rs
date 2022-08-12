use std::iter::FromIterator;

impl Solution {
    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        let mut stack = vec![];
        
        for &key in rooms[0].iter(){
            stack.push(key);
        }
        
        rooms[0].clear();
        
        while !stack.is_empty(){
            let i = stack.pop().unwrap() as usize;
            
            if rooms[i].len() > 0{
                
                for &key in rooms[i].iter(){
                    stack.push(key);
                }
                
                rooms[i].clear();
            }
        }
        
        rooms.into_iter().all(|v| v.is_empty())
    }
}