use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map:HashMap<char, String> = HashMap::new();
        let pattern = pattern.chars().collect::<Vec<char>>();
        let s = s.split_whitespace().map(|a|{a.to_string()}).collect::<Vec<String>>();
        
        if pattern.len() != s.len(){
            return false;
        }
        
        for i in 0..pattern.len(){
            let c = pattern[i];
            let word = &s[i];
            
            if let Some(storedWord) = map.get(&c){
                if storedWord != word{
                    return false;
                }
            }else{
                map.insert(c, word.to_string());
            }            
        }
        
        let mut values = map.into_values().collect::<Vec<String>>();
        values.sort();
        
        for i in 0..values.len() - 1{
            if values[i] == values[i + 1]{
                return false;
            }
        }
        true
    }
}