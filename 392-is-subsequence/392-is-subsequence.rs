impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len(){
            return false;
        }
        
        if s.len() == 0{
            return true;
        }
        
        let s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        for c in t.chars(){
            if s[i] == c{
                i +=1;
            }
            
            if i == s.len(){
                return true;
            }
        }
        
        false
    }
}