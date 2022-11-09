use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut ordered_set = BTreeSet::from_iter(mat[0].clone());
        let mut removes = vec![];
        
        for arr in mat.into_iter().skip(1){
            let set: HashSet<i32> = HashSet::from_iter(arr);
            
            for n in ordered_set.iter().cloned(){
                if !set.contains(&n){
                    removes.push(n);
                }
            }
            
            while let Some(n) = removes.pop(){
                ordered_set.remove(&n);
            }
        }
        
        ordered_set.into_iter().next().unwrap_or(-1)
    }
}