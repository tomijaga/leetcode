use std::{
    cmp::max,
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
        
        // println!("{:?}", &map);
        
        
        let mut points = 0;
        let mut start = 0;
        
        let mut s = s.chars().collect::<Vec<char>>();
        let mut sub = sub.chars().collect::<Vec<char>>();
        
        for i in 0..=(s.len() - sub.len()){
            let tmp_points = get_points(&s, &sub, i);
            
            if tmp_points > points{
                points = tmp_points;
                start = i;
            }
        }
        
        if points ==  sub.len() as i32{
            return true;
        }
        
        for i in 0..sub.len(){
            if let Some(set) = map.get(&sub[i]){
                let prev = sub[i]; 
                
                if let Some(&c) = set.get(&s[start + i]){
                    sub[i] = c;
                    // println!("{:?}", sub[i]);
                }
            }
        }
        
        get_points(&s, &sub, start) == sub.len() as i32
    }
}

pub fn get_points(s: &Vec<char>, sub: &Vec<char>, start: usize)-> i32{
    let mut s_chars = s.iter().skip(start);
    let mut sub_chars = sub.iter();
    
    let mut points = 0;
    for sub_char in sub_chars{
        let s_char = s_chars.next().unwrap();
        
        if s_char == sub_char{
            points +=1;
        }
    }
    
    points
}