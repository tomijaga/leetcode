impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0{
            return false;
        }
        
        let s = x.to_string().chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = s.len() -1;
        
        while(i<j){
            if (s[i]!=s[j]){
                return false;
            }
            i+=1;
            j-=1;
        }
        true
    }
}