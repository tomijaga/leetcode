impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        
        let (mut left, mut right) = (0, s.len() - 1);
        
        while left < right && right < s.len(){
            while left < right && !s[left].is_alphanumeric(){
                left +=1;
            }
            
            while left < right && !s[right].is_alphanumeric(){
                right -=1;
            }
            
            if to_lowercase(s[left]) != to_lowercase(s[right]){
                return false;
            }
            
            left+=1;
            right -=1;
        }
        
        true
    }
}

pub fn to_lowercase(c: char) -> char {
    c.to_lowercase().next().unwrap()
}