impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut max = 0;
        
        for i in 0..words.len(){
            for j in i+ 1..words.len(){
                if is_valid(words[i].as_bytes(), words[j].as_bytes()){
                    let n = words[i].len() * words[j].len();
                    
                    if n > max{
                        max = n;
                    }
                }
            }
        }
        
        max as i32
    }
}

pub fn is_valid(a: &[u8], b: &[u8])-> bool{
    for i in 0..a.len(){
        for j in 0..b.len(){
            if a[i] == b[j]{
                return false;
            }
        }
    }
    
    return true;
}