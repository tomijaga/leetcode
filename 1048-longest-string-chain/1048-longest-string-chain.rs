use std::{
    cmp::max,
    collections::HashMap
};

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        words.sort_unstable_by(|a, b|{a.len().cmp(&b.len())});
        
        for word in words{
            let bytes = word.as_bytes().to_vec();
            let mut chain = 0;
            
            for i in 0..bytes.len(){
                let mut successor = bytes.clone();
                successor.remove(i);
                
                if let Some(&succ_chain) = map.get(&successor){
                    chain = max(succ_chain, chain);
                }
            }
            
            map.insert(bytes, chain + 1);
            
        }

        let (_, max_chain) = map
            .into_iter()
            .max_by(|(_, n), (_, n1)|{
                n.cmp(&n1)
            })
            .unwrap();
        
        max_chain
    }
}