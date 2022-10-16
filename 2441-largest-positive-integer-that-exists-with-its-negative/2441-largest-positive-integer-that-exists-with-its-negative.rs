use std::collections::{BTreeSet, HashSet};

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut positive = BTreeSet::new();
        let mut negative = HashSet::new();
        
        for n in nums{
            if n > 0{
                positive.insert(n);
            }else{
                negative.insert(n);
            }
        }
        
        for n in positive.into_iter().rev(){
            if negative.contains(&(-n)){
                return n;
            }
        }
        
        -1
    }
}