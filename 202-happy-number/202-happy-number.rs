use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        
        while !set.contains(&n){
            set.insert(n);
            let s = n.to_string();
            
            n = s.split("").map(|c|{
                c.parse::<i32>()
                .unwrap_or_default()
                .pow(2)
            }).sum();
            
            // println!("n: {:?}", n);
            if n == 1{
                return true;
            }
        }
        
        false
    }
}