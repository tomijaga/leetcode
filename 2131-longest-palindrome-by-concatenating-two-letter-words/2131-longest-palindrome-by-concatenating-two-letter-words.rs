use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        
        let mut cnt = 0;
        
        for word in words{
            let word: Vec<char> = word.chars().collect();
            let flipped = vec![word[1], word[0]];
            
            if word == flipped{
                if set.contains(&word){
                    set.remove(&word);
                    cnt+=2;
                }else{
                    set.insert(word.clone());
                }
                
                continue;
            }
            
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
                map.insert(word.clone(), 1);
            }
        }
        
        (cnt * 2 ) + if set.len() > 0 {2} else {0}
    }
}