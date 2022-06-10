use std::{
    collections::HashMap,
    cmp
};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        
        let mut max = 0;
        let mut i = 0;
        
        for (j, c) in s.chars().enumerate(){
            if let Some(dup_index) = map.get(&c){
                i = cmp::max(dup_index + 1, i);
            }
                
            map.insert(c, j);
            max = cmp::max(max, j - i + 1);
        }
        
        max as i32
    }
}
