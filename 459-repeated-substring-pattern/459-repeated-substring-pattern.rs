impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        
        for n in 1..(chars.len()/2 + 1){
            if (chars.len() % n > 0){
                continue;
            }
            
            let mut start = n;
            let mut prev = chars[0..n].to_vec();
            let mut curr = vec![];
            
            let mut repeated = true;
            
            while(repeated && start < chars.len()){
                
                if (start + n > chars.len()){
                    repeated = false; 
                    break;
                }
                
                curr = chars[start..(start + n)].to_vec();
                start += n;
                
                // println!("{:?}, {:?}", prev, curr);
                
                repeated = prev == curr && repeated;
                prev = curr;
            }
            
            if (repeated == true){
                return true;
            }
            
        }
        
        false
    }
}