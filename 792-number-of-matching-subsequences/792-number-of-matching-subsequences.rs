use std::collections::HashMap;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        
        for word in words.iter(){
            map.entry(word.chars().next().unwrap())
                .or_insert(vec![])
                .push(&word[..]);
            
        }
        
        let mut cnt = 0;
        
        for c in s.chars(){
            
            if let Some(slices) = map.remove(&c){
                for slice in slices{
                    let next_slice = &slice[1..];
                    
                    if next_slice.len() == 0{
                        cnt +=1;
                    }else{
                        map.entry(next_slice.chars().next().unwrap())
                            .or_insert(vec![])
                            .push(next_slice);
                    }
                }
            }
        }
        
        cnt 
    }
}