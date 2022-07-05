use std::{
    collections::HashSet,
    iter::FromIterator,
    
};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set : HashSet<i32> = HashSet::from_iter(nums.into_iter());
        
        let mut max_cnt = 0;
        
        while set.len() > 0{
            let n = *set.iter().next().unwrap();
                set.remove(&n);
                
                let mut cnt = 1;
                
                let mut curr = n + 1;
                while set.contains(&curr){
                    curr+=1;
                    cnt +=1;
                    set.remove(&n);
                }
                
                curr = n - 1;
                while set.contains(&curr){
                    curr-=1;
                    cnt+=1;
                    set.remove(&n);
                }
                
                max_cnt = max_cnt.max(cnt);

        }
        
        max_cnt
    }
}