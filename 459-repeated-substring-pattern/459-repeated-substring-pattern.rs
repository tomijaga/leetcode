pub fn match_all_subs(bytes: &[u8], len: usize) -> bool{
    let sub = &bytes[..len];
    
    (&bytes[len..]).chunks(len).all(|chunk| chunk == sub)
}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let bytes = s.as_bytes();
        
        for n in 1..(bytes.len()/2 + 1){
            if (bytes.len() % n > 0){
                continue;
            }
            
            if (match_all_subs(bytes, n)){
                return true;
            }
        }
        
        false
    }
}