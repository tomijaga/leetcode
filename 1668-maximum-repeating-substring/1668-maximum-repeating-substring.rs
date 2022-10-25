use std::collections::HashMap;

impl Solution {
    pub fn max_repeating(mut sequence: String, mut word: String) -> i32 {
        let mut cnt = 0;
        let mut words = vec![word.clone()];
        
        while let Some(_) =  sequence.find(&(words.concat())){
            words.push(word.clone());
            cnt+=1;
        }
        
        cnt
    }
}