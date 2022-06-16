use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut map: HashMap<u8, HashSet<&str>> = HashMap::new();
        
        for idea in ideas.iter() {
            let first_char = idea.as_bytes()[0];
            map.entry(first_char).or_insert(HashSet::new()).insert(&idea[1..]);
        }
        
        let mut res = 0;
        for i in b'a'..=b'z' {
            for j in i+1..=b'z' {
                if !map.contains_key(&i) || !map.contains_key(&j) {
                    continue;
                }
                let mut set_a = map.get(&i).unwrap();
                let mut set_b = map.get(&j).unwrap();
                
                res += 2 * (set_a - set_b).len() * (set_b - set_a).len();
            }
        }
        
        res as i64
    }
}