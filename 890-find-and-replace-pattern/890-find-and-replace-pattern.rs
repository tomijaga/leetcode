use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut map = vec!['0'; 26];
        let mut p_map = vec!['0'; 26];
        
        let mut res:Vec<String> = vec![];

        for word in words{
            let mut is_match = true;
            for (w, p) in word.chars().zip(pattern.chars()){
                if map[id(w)] == '0'{
                    map[id(w)] = p;
                }
                
                if p_map[id(p)] == '0'{
                    p_map[id(p)] = w;
                }
                
                if map[id(w)] != p || p_map[id(p)] != w{
                    is_match = false;
                    continue;
                }
            }
            
            map = vec!['0'; 26];
            p_map = vec!['0'; 26];
            
            if is_match{
                res.push(word);
            }
        }
        
        res
        
    }
}

pub fn id(c: char) -> usize{
    c as usize - 'a' as usize
}