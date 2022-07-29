use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut map = HashMap::new();

        let mut mapped_pattern = vec![0; pattern.len()];
        let mut mapped_word = vec![0; pattern.len()];
        
        let mut res:Vec<String> = vec![];
        
        let mut cnt = 0;
        for (i, c) in pattern.chars().enumerate(){
            if !map.contains_key(&c){
                map.insert(c, cnt);
                cnt+=1;
            }
                
            mapped_pattern[i] = *map.get(&c).unwrap();
        }
        
        for word in words{
            map.clear();
            
            cnt = 0;
            for (i, c) in word.chars().enumerate(){
                if !map.contains_key(&c){
                    map.insert(c, cnt);
                    cnt+=1;
                }
                
                mapped_word[i] = *map.get(&c).unwrap();
            }
            
            if mapped_word == mapped_pattern{
                res.push(word);
            }
        }
        
        res
        
    }
}