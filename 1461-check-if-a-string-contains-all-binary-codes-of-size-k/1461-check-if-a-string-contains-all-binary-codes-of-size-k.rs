use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut set = HashSet::new();
        
        let combinations = 2_usize.pow(k as u32);
        let end = s.len() as i32 - k as i32 + 1;
        for i in 0..end{
            let i = i as usize;
            let slice = &s[i.. i + k];
            
            set.insert(slice);
            
            if set.len() == combinations{
                return true;
            }
        }
        println!("{:?}", set);
        return false;
    }
}