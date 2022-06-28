use std::collections::{HashMap, HashSet, BinaryHeap};

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut map = HashMap::new();
        
        for c in s.chars(){
            *map.entry(c).or_insert(0) +=1; 
        }
        
        let mut v: BinaryHeap<i32> = map.into_iter().map(|(_, n)|{n}).collect();
        let mut set = HashSet::new();
        
        let mut count = 0;
        while !v.is_empty(){
            let mut n = v.pop().unwrap();
            if n == 0{
                continue;
            }
            
            if set.contains(&n){
                println!("{:?}", n);
                n-=1;
                count+=1;
                v.push(n);
            }else{
                set.insert(n);
            }
        }
        
        count
    }
}