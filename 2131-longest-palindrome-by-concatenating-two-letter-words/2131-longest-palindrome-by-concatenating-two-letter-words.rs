use std::collections::BTreeMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut map = BTreeMap::new();
        
        let mut unpaired_dup_chars = 0;        
        let mut cnt = 0;
        
        for word in words{
            let mut chars = word.chars();
            let a = id(chars.next().unwrap());
            let b = id(chars.next().unwrap());
            
            let mut passed = false;
            if let Some(n) = map.get_mut(&(a, b)){
                if *n > 0{
                    *n-=1;
                    cnt+=4;
                    if a == b{
                        unpaired_dup_chars-=1;
                    }
                    passed = true
                }
            }
            
            if !passed{
                if a == b{
                    unpaired_dup_chars +=1;
                }
                *map.entry((b, a)).or_insert(0) +=1;
            }
        }
        
        cnt + if unpaired_dup_chars > 0 {2} else {0}
    }
}

pub fn id(c: char) -> usize{
    c as usize - 'a' as usize
}