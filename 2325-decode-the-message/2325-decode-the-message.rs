

use std::collections::HashMap;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut i:usize = 0;
        
        let mut map = vec!['-'; 26];
        let mut message = message.chars().collect::<Vec<char>>();    
        
        let mut alphabets:Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        
        for c in key.chars(){
            if c != ' ' && map[id(c)] == '-'{
                map[id(c)] = alphabets[i];
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
            
            *c = map[id(*c)];
        }
        message.into_iter().collect::<String>()
    }
}

pub fn id(c: char)-> usize{
    c as usize - 'a' as usize
}