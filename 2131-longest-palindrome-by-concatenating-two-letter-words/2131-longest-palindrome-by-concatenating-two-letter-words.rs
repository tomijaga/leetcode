
use std::collections::{ HashMap };

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut map = vec![vec![0; 26]; 26];
        let mut dup_chars_unpaired = 0;        
        let mut cnt = 0;
        
        let words_iter = words.into_iter()
            .map(|w| {
                w.chars()
                    .map(|c| id(c))
                    .collect::<Vec<usize>>()
        });
        
        for word in words_iter{
            let ptr = &mut map[word[0]][word[1]];
            if word[0] == word[1]{
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
                map[word[1]][word[0]] += 1;
            }
        }
        
        cnt + if dup_chars_unpaired > 0 {2} else {0}
    }
}

pub fn id(c: char) -> usize{
    c as usize - 'a' as usize
}