use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        
        for n in nums1.iter(){
            map.insert(*n, false);
        }
        
        let mut set = HashSet::new();
        
        for n in nums2{
            if let Some(exists) = map.get_mut(&n){
                if (*exists == false){
                    *exists = true;
                }
            }else{
                set.insert(n);
            }
        }
        
        let mut v1 = vec![];
        
        for (key, val) in map{
            if (val == false){
                v1.push(key);
            }
        }
        
        vec![v1, set.into_iter().collect::<Vec<i32>>()]
    }
}