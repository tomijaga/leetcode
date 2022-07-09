impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        
        let (mut left, mut right) = (0, s.len() - 1);
        
        let len = s.len();
        
        for i in 0..(s.len()/2){
            if s[i] != s[len - i - 1]{
                return false
            }
        }

        true
    }
}