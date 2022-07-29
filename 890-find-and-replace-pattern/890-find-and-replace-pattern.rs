use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut map = HashMap::new();

        let mut mapped_pattern:Vec<(i32, i32)> = vec![];
        
        let mut res:Vec<String> = vec![];
        
        let mut cnt = 0;
        for c in pattern.chars(){
            if !map.contains_key(&c){
                map.insert(c, cnt);
                cnt+=1;
            }
            
            let n = *map.get(&c).unwrap();
                
            if let Some(last) = mapped_pattern.last_mut() {
                if last.0 == n{
                    last.1 +=1;
                }else{
                    mapped_pattern.push((n, 1));
                }
            }else{
                mapped_pattern.push((n, 1));
            }
        }
        
        let mut mapped_word:Vec<(i32, i32)> = Vec::with_capacity(mapped_pattern.len());
        
        for word in words{
            map.clear();
            
            cnt = 0;
            let mut i = 0;
            for c in word.chars(){
                if !map.contains_key(&c){
                    map.insert(c, cnt);
                    cnt+=1;
                }
                
                let n = *map.get(&c).unwrap();
                
                if let Some(last) = mapped_word.last_mut() {
                    if last.0 == n{
                        last.1 +=1;
                    }else{
                        mapped_word.push((n, 1));
                    }
                }else{
                    mapped_word.push((n, 1));
                }
                
                if mapped_word.len() > mapped_pattern.len(){
                    break;
                }
            }
            
            if mapped_word == mapped_pattern{
                res.push(word);
            }
            
            mapped_word.clear();
        }
        
        res
        
    }
}