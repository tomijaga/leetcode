use std::{
    collections::HashSet,
    iter::FromIterator,
    
};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set : HashSet<i32> = HashSet::from_iter(nums.into_iter());
        
        let mut max_cnt = 0;
        
        while set.len() > 0{
            if let Some(&n) = set.iter().next(){
                let mut cnt = 1;
                
                set.remove(&n);
                dfs(&mut set, &mut (cnt), 0, n);
                
                max_cnt = max_cnt.max(cnt);
            }
        }
        
        max_cnt
    }
}

pub fn dfs(set: &mut HashSet<i32>, cnt: &mut i32, dir: i32, n: i32){
    
    if dir != 2{
        let new_n = n+1;
        if set.contains(&new_n){
            *cnt += 1;
            set.remove(&new_n);
            dfs(set, cnt, 1, new_n);
        }
    }
    
    if dir != 1{
        let new_n = n - 1;
        if set.contains(&new_n){
            *cnt += 1;
            set.remove(&new_n);
            dfs(set, cnt, 2, new_n);
        }
    }
}