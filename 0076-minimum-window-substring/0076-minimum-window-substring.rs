use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {

        if s.len() < t.len() {
            return String::new();
        }
        
        let mut map_s: HashMap<char, i32> = HashMap::new();
        let mut map_t: HashMap<char, i32> = HashMap::new();
        
        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();
        let mut required = 0;
        let mut found = 0;
        
        for &c in t.iter(){
            *map_t.entry(c).or_default() +=1;
            map_s.entry(c).or_default();
        }
        
        required = map_t.len() as i32;
        
        let mut min_tup = (usize::MAX, 0);
        let mut j = 0;
        
        for i in 0..s.len(){
            
            while found < required && j < s.len(){
                let c = s[j];
                
                if let Some(cnt) = map_s.get_mut(&c){
                    *cnt+= 1;
                    
                    if map_s[&c] == map_t[&c]{
                        found += 1;
                    }
                }
                
                j+=1;
            }
            
            // println!("{:?}", (i, j, found));
            
            if found == required{
                let len = j - i;

                if len < min_tup.0 {
                    min_tup.0 = len;
                    min_tup.1 = i;
                }
            }
            
            
            let c = s[i];
            
            if let Some(cnt) = map_s.get_mut(&c){
                if *cnt == map_t[&c]{
                    found -= 1;
                }
                
                if *cnt > 0{
                    *cnt -= 1;
                }
            }
        }
        
        let (len, start) = min_tup;
        
        if len == usize::MAX{
            String::new()
        }else{
            (&s[start..(start + len)]).into_iter()
            .collect::<String>()
        }
    }
}
