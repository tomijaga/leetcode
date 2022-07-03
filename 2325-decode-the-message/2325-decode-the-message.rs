use std::collections::HashMap;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut i:u8 = 0;
        
        let mut map = vec![0; 26];
        let mut message = message.chars().collect::<Vec<char>>();    
        
        for c in key.chars(){
            if c != ' ' && map[id(c)] == 0{
                map[id(c)] = i + 97;
                i+=1;
            }
            
            if i == 26{
                break;
            }
        }
        
        for c in message.iter_mut(){
            if *c == ' '{
                continue;
            }
            
            *c = map[id(*c)] as char;
        }
        message.into_iter().collect::<String>()
    }
}

pub fn id(c: char)-> usize{
    c as usize - 'a' as usize
}