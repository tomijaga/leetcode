use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut map = HashMap::new();
        let mut p_map = HashMap::new();
        
        let mut res:Vec<String> = vec![];

        for word in words{
            let mut is_match = true;
            for (w, p) in word.chars().zip(pattern.chars()){
                map.entry(w).or_insert(p);
                p_map.entry(p).or_insert(w);
                
                if map.get(&w) != Some(&p) || p_map.get(&p) != Some(&w){
                    is_match = false;
                    continue;
                }
            }
            
            map.clear();
            p_map.clear();
            
            if is_match{
                res.push(word);
            }
        }
        
        res
        
    }
}