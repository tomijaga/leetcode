use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s: String, mut p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }
        
        let mut s = s.chars().collect::<Vec<char>>();
        
        let mut res = vec![];
        
        let mut sMap = HashMap::new();
        let mut pMap = HashMap::new();
        
        for (a, &b) in p.chars().zip(s.iter()){
            *pMap.entry(a).or_insert(0) +=1;
            *sMap.entry(b).or_insert(0) +=1;
        }
        
        // println!("{:?}", (&sMap, &pMap));
        
        for i in 0..=(s.len() - p.len()){
            if is_anagram(&sMap, &pMap){
                res.push(i as i32);
            }
            
            *sMap.get_mut(&s[i]).unwrap()-=1;
            if i + p.len() < s.len(){
                *sMap.entry(s[i + p.len()]).or_insert(0) +=1;
            }
        }
        
        res
    }
}

pub fn is_anagram(sMap: &HashMap<char, i32>, pMap: &HashMap<char, i32>) -> bool{
    for (c, cnt) in pMap{
        
        if let Some (cnt2) = sMap.get(&c){
            if cnt != cnt2{
                return false;
            }
        }else{
            return false;
        }
    }
    
    return true;
}