use std::collections::HashSet;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.iter().sum::<i32>() <target{
            return vec![];
        }
        
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let mut store: HashSet<Vec<i32>> = HashSet::new();
        
        fn cbns(set: &mut HashSet<Vec<i32>>, store: &mut HashSet<Vec<i32>>, candidates: &[i32], tuple:(Vec<i32>, i32), target: i32){
            if candidates.len() == 0{
                return;
            }
            
            let (nums, sum) = tuple;
            
            for (i, &n) in candidates.iter().enumerate(){
                let sum = sum + n;
                
                if sum <= target{
                    let mut new_nums = nums.clone();
                    new_nums.push(n);
                    
                    println!("{:?}", (&new_nums, sum));
                    new_nums.sort_unstable();
                    if store.contains(&new_nums){
                        continue;
                    }else{
                        store.insert(new_nums.clone());
                    }
                    
                    if sum == target{
                        set.insert(new_nums);
                    }else{
                        let j = i + 1;
                        if j < candidates.len(){
                            cbns(set, store, &candidates[j..], (new_nums, sum), target);
                        }
                    }
                }
            }
        }
        
        cbns(&mut set, &mut store, &candidates, (vec![], 0), target );
        
        set.into_iter().collect::<Vec<Vec<i32>>>()
    }
}