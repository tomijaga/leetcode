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
        
        let mut matches = 0;
        
        for (&a, &b) in map1.iter().zip(map2.iter()){
            if a == b {
                matches +=1;
            }
        }
        
        if matches == 26{
            return true;
        }
        
        for (prev, curr) in s2.chars().zip(s2.chars().skip(len)){
            if map2[id(prev)] == map1[id(prev)]{
                matches -=1;
            }
            
            map2[id(prev)] -= 1;
            
            if map2[id(prev)] == map1[id(prev)]{
                matches +=1;
            }
            
            if map2[id(curr)] == map1[id(curr)]{
                matches -=1;
            }
            
            map2[id(curr)] += 1;
            
            if map2[id(curr)] == map1[id(curr)]{
                matches +=1;
            }

            if matches == 26{
                return true;
            }
        }
        
        false
    }
}

fn id(c: char) -> usize{
    (c as u8 - 'a' as u8) as usize
}