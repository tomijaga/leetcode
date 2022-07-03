use std::collections::HashMap;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut i:usize = 0;
        
        let mut map = HashMap::from([(' ', ' ')]);
        let mut message = message.chars().collect::<Vec<char>>();    
        
        let mut alphabets:Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        
        for c in key.chars(){
            if c != ' ' && map.get(&c).is_none(){
                map.insert(c, alphabets[i]);
                i+=1;
            }
            
            if i == 26{
                break;
            }
        }
        
        for c in message.iter_mut(){
            *c = *map.get(c).unwrap();
        }
        
        message.into_iter().collect::<String>()
    }
}