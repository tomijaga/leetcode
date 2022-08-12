use std::iter::FromIterator;

impl Solution {
    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        let mut stack = vec![];
        
        for &key in rooms[0].iter(){
            stack.push(key);
        }
        
        rooms[0].clear();
        
        let mut sum = 1;
        
        while !stack.is_empty(){
            let i = stack.pop().unwrap() as usize;
            
            if rooms[i].len() > 0{
                sum +=1;
                
                for &key in rooms[i].iter(){
                    stack.push(key);
                }
                
                rooms[i].clear();
            }
        }
        
        rooms.into_iter().all(|v| v.is_empty())
    }
}