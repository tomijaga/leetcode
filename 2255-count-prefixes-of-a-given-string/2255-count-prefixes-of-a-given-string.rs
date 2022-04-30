impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut n = 0;
        
        for prefix in words{
            if (s.starts_with(&prefix)){
                n+=1;
            }
        }
        
        return n;
    }
}