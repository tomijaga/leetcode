use std::{
    collections::{
        HashMap,
        HashSet
    }
};

impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let mut map: HashMap<char, HashSet<char>> = HashMap::new();
        
        for v in mappings{
            map.entry(v[0]).or_insert(HashSet::new()).insert(v[1]);
        }
        
        let mut matches = 0;
        let mut start = 0;
        
        let mut s = s.chars().collect::<Vec<char>>();
        let mut sub = sub.chars().collect::<Vec<char>>();
        
        for i in 0..=(s.len() - sub.len()){
            let tmp = get_matches(&s, &sub, i);
            
            if tmp > matches{
                matches = tmp;
                start = i;
            }
        }
        
        if matches ==  sub.len() as i32{
            return true;
        }
        
        for i in 0..sub.len(){
            if let Some(set) = map.get(&sub[i]){
                let prev = sub[i]; 
                
                if let Some(&c) = set.get(&s[start + i]){
                    sub[i] = c;
                }
            }
        }
        
        get_matches(&s, &sub, start) == sub.len() as i32
    }
}

// number of matching chars from the start index
pub fn get_matches(s: &Vec<char>, sub: &Vec<char>, start: usize)-> i32{
    let mut s_chars = s.iter().skip(start);
    let mut sub_chars = sub.iter();
    
    let mut matches = 0;
    for sub_char in sub_chars{
        let s_char = s_chars.next().unwrap();
        
        if s_char == sub_char{
            matches +=1;
        }
    }
    
    matches
}