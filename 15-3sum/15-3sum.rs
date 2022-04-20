use std::collections::HashMap;
use std::collections::HashSet;


pub fn exists(nums: &Vec<Vec<i32>>, elem: &Vec<i32> ) -> bool{
    for n in nums{
        if (n[0]== elem[0] && n[1]== elem[1] && n[2]== elem[2]){
            return true;
        }
    }
    return false;
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        let mut targets = HashMap::new();
        // let mut sum = vec![];
        let mut sum:HashSet<Vec<i32>> = HashSet::new();
        
        for (i, n) in nums.iter().enumerate(){
            targets.insert(n, i);
        }
        
        for i in 0..nums.len(){
            for j in (i + 1)..nums.len(){
                let target = 0 - ( nums[i] + nums[j]);
                
                if let Some (index) = targets.get(&target){
                    if (i != j && j!= *index && *index!=i){
                        let mut elem = vec![nums[*index], nums[i], nums[j]];
                        elem.sort();
                        sum.insert(elem);
                    }
                }
            }
        } 
        
        sum.into_iter().collect::<Vec<Vec<i32>>>()
    }
}