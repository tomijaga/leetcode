pub fn bytes_match(a: &[u8], b: &[u8])->i32{
    for i in 0..(a.len() - (b.len() -1)){
        if a[i..(i + b.len())] == *b{
            return i as i32;
        }
    }
    return -1;
}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() >= needle.len(){
            bytes_match(haystack.as_bytes(), needle.as_bytes())
        }else{
            -1
        }
    }
}