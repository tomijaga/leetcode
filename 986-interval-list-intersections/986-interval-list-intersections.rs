use std::cmp::{
    max, min
};

impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if first_list.is_empty() || second_list.is_empty(){
            return vec![];
        }
        
        let mut res = vec![];
        let (mut i, mut j) = (0, 0);
        
        while (i < first_list.len() && j < second_list.len()){
            let (v1, v2)= (&first_list[i], &second_list[j]);
            
            let l = max(v1[0], v2[0]);
            let r = min(v1[1], v2[1]);
            
            if l <= r{
                res.push(vec![l, r]);
            }
            
            if v1[1] < v2[1]{
                i+=1;
            } else{
                j +=1;
            }
        }
        
        res
    }
}