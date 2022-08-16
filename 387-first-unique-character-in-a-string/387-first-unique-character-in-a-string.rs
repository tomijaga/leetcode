use std::collections::BTreeMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let (EMPTY, DUPLICATE) = (i16::MIN, -1);
        
        let mut v = vec![EMPTY; 26];
        
        for (i, c) in s.chars().enumerate(){
            if v[c as usize - 'a' as usize] == EMPTY{
                v[c as usize - 'a' as usize] = i as i16;
            }else{
                v[c as usize - 'a' as usize] = DUPLICATE;
            }
        }
        
        let mut min_index = i16::MAX;
        
        for i in v{
            if i != DUPLICATE && i != EMPTY{
                min_index = min_index.min(i);
            }
        }
        
        if min_index == i16::MAX{
            -1
        }else{
            min_index as i32
        }
    }
}