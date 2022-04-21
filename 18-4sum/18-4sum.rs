use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums =nums;
        nums.sort();
        let l = nums.len();
        let mut results: HashSet<Vec<i32>> = HashSet::new();
        
        if l<4 {
            return vec![]
        }
        
        for a in 0..l-3{
            for b in (a+1)..l-2{
                let mut c = b + 1;
                let mut d = l - 1;
                
                while(c<d){
                    // println!("{:?}", vec![a, b, c, d]);
                    
                    let sum = nums[a] + nums[b] + nums[c] + nums[d];
                    // println!("{:?}", vec![nums[a], nums[b], nums[c], nums[d]]);
                    if (sum > target){
                        d-=1;
                    }else if(sum < target){
                        c+=1;
                    }else{
                        results.insert(vec![nums[a], nums[b], nums[c], nums[d]]);
                        c+=1;
                    }
                }
            }
        }
        results.into_iter().collect::<Vec<Vec<i32>>>()
    }
}