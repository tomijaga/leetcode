use std::collections::{HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut main_set: HashSet<i32> = HashSet::from_iter(mat[0].clone());
        let mut removes = vec![];
        
        for arr in mat.into_iter().skip(1){
            let set: HashSet<i32> = HashSet::from_iter(arr);
            
            for n in main_set.iter().cloned(){
                if !set.contains(&n){
                    removes.push(n);
                }
            }
            
            while let Some(n) = removes.pop(){
                main_set.remove(&n);
            }
        }
        
        let mut min_val = i32::MAX;
        
        for n in main_set{
            min_val = min_val.min(n);
        }
        
        if min_val == i32::MAX { -1 }else { min_val }
    }
}