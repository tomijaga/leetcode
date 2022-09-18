impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s:Vec<char> = s.chars().collect();
        let mut max_len = 0;
        let mut len = 1;
        
        for w in s.windows(2){
            let a = w[0];
            let b = w[1];
            
            if b as u8 == a as u8 + 1{
                len += 1;
            }else{
                if len > max_len{
                    max_len = len;
                }
                len = 1;
            }
        }
        
        max_len.max(len)
    }
}