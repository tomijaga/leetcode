pub fn k_duplicates(s: &String, start: usize, end: usize)->bool{
    if end > s.len(){
        return false;
    }
    let slice = &s[start..end];
    
    let slice = &s[start..end].as_bytes();
    
    let elem = slice[0];
    slice.iter().all(|&a|{a == elem})
}

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s = s;
        
        if s.len() <= k as usize{
            return s;
        }
        
        let mut dup_found = true;
        
        dup_found = false;
        for start in (0..=(s.len() - k)).rev(){
            let end = start + k;
                
            if k_duplicates(&s, start, end){
                dup_found = true;
                // println!("dup -> {:?}", &s[start..end]);
                
                s = s[0..start].to_string() + if end < s.len() { &s[end..]} else {""};
                // println!("s: {:?}", s);
                
            }
        }
        
        s
    }
}