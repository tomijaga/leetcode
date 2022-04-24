use std::collections::{HashSet};

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set1:HashSet<i32> = nums1.into_iter().collect();
        let mut set2:HashSet<i32> = nums2.into_iter().collect();
        
        let (mut v1, mut v2) = (vec![], vec![]);
        
        for n in set1.iter(){
            if !set2.contains(n){
                v1.push(*n);
            }
        }
        
        for n in set2{
            if !set1.contains(&n){
                v2.push(n);
            }
        }
    
        vec![v1, v2]
    }
}