impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let bytes = num.as_bytes();
        
        let mut exists = false;
        let mut max = u8::MIN;
        
        for i in 0..bytes.len()-2{
            if bytes[i] == bytes[i + 1] && bytes[i + 1] == bytes[i + 2]{
                let n = bytes[i] - 48; // ascii digits range from 48 - 58
                if n >= max{
                    max = n;
                    exists = true;
                }
            }
        }
        
        if exists{
            let vec = vec![max.to_string(); 3];
            vec.into_iter().collect::<String>()
        }else{
            String::new()
        }
    }
}