use std::collections::HashMap;
use std::str;

pub fn encode(str: &String)-> String{
    let mut chars = str.chars().collect::<Vec<char>>();
    chars.sort();
    
    chars.into_iter().collect::<String>()
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map:HashMap<String, Vec<String>> = HashMap::new();
        
        for str in strs{
            let key = encode(&str);
            
            if let Some(vals) = map.get_mut(&key){
                vals.push(str);
            }else{
                map.insert(key, vec![str]);
            }
        }
        
        map.into_values().collect::<Vec<Vec<String>>>()
    }
}