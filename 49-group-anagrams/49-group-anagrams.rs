use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u16; 26], Vec<String>> = HashMap::new();
        
        for s in strs{
            let key =  count_chars(&s);
            
            if let Some(vals) = map.get_mut(&key){
                vals.push(s);
            }else{
                map.insert(key, vec![s]);
            }
        }
        
        map.into_values().collect::<Vec<Vec<String>>>()
    }
}

pub fn count_chars(s: &str) -> [u16; 26]{
    let mut arr = [0_u16; 26];
    
    for c in s.chars(){
        arr[(c as u8 - 'a' as u8) as usize] += 1;
    }
    
    arr
}