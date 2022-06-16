
use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut sets =  vec![HashSet::new(); 26];
        
        for idea in ideas.iter(){
            let bytes = idea.as_bytes();
            sets[(bytes[0] - b'a') as usize].insert(&idea[1..]);
        }
        
        let mut res = 0;
        for i in 0..sets.len() - 1{
            for j in (i+1)..sets.len(){
                let (a, b) = (&sets[i], &sets[j]);
                
                let duplicates = a.intersection(b).collect::<HashSet<_>>().len();
                res += 2_i64 * ((a.len() - duplicates) * (b.len() - duplicates)) as i64;
            }   
        }
        
        res
    }
}