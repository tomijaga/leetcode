
use std::collections::{ HashMap };

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        let mut dup_chars_unpaired = 0;        
        let mut cnt = 0;
        
        for word in words.into_iter().map(|w| w.chars().collect::<Vec<char>>()){
            
            if word[0] == word[1]{
                
                if let Some(n) = map.get_mut(&word){
                    if *n > 0{
                        *n-=1;
                        dup_chars_unpaired-=1;
                        cnt+=2;
                    }else{
                        *n += 1;
                        dup_chars_unpaired+=1;
                    }
                }else{
                    map.insert(word, 1);
                    dup_chars_unpaired +=1;
                }
                
                continue;
            }
            
            let flipped = vec![word[1], word[0]];
            
            if let Some(n) = map.get_mut(&flipped){
                *n -= 1;
                
                if *n >= 0{
                    cnt +=2;
                }
                
            }else if let Some(n) = map.get_mut(&word){
                *n += 1;
                
                if *n <= 0{
                    cnt +=2;
                }
            }else{
                map.insert(word, 1);
            }
        }
        
        (cnt * 2 ) + if dup_chars_unpaired > 0 {2} else {0}
    }
}