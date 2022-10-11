impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        
        if s1.len() > s2.len(){
            return false;
        }
        
        let mut map1 = vec![0; 26];
        let mut map2 = vec![0; 26];
        let len = s1.len();
        
        for (a, b) in s1.chars().zip(s2.chars()).take(len){
            map1[id(a)] += 1;
            map2[id(b)] += 1;
        }
        
        if map1 == map2{
            return true;
        }
        
        let mut s2_iter = s2.chars();
        
        for (prev, curr) in s2_iter.clone().zip(s2_iter.skip(len)){
            map2[id(prev)] -= 1;
            map2[id(curr)] += 1;
            
            if map1 == map2{
                return true;
            }
        }
        
        false
    }
}

fn id(c: char) -> usize{
    (c as u8 - 'a' as u8) as usize
}