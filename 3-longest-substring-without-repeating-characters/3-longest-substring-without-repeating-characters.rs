use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut max_len = 0;
        let mut i = 0;
        
        for (j, c) in s.chars().enumerate(){
            if let Some(&dup_index) = map.get(&c){
                i = i.max(dup_index + 1);
            }
            
            map.insert(c, j);
            max_len = max_len.max(j - i + 1);
        }

        max_len as i32
    }
}

fn id(c: char) -> usize{
    (c as u8 - 'a' as u8) as usize
}