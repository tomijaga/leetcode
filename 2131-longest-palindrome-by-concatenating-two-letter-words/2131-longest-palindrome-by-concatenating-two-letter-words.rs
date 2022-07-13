
use std::collections::{ HashMap };

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut map = vec![vec![0; 26]; 26];
        let mut dup_chars_unpaired = 0;        
        let mut cnt = 0;
        
        for word in words{
            let mut chars = word.chars();
            let a = id(chars.next().unwrap());
            let b = id(chars.next().unwrap());
            
            let ptr = &mut map[a][b];
            if a == b{
                if *ptr > 0{
                    *ptr -=1;
                    cnt+=4;
                    dup_chars_unpaired-=1;
                }else{
                    *ptr +=1;
                    dup_chars_unpaired +=1;
                }
                continue;
            }
            if *ptr > 0{
                *ptr -=1;
                cnt+=4;
            }else{
                map[b][a] += 1;
            }
        }
        
        cnt + if dup_chars_unpaired > 0 {2} else {0}
    }
}

pub fn id(c: char) -> usize{
    c as usize - 'a' as usize
}