use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(mut opts: Vec<i32>) -> Vec<Vec<i32>> {
        pub fn sub(opts: &Vec<i32>, i: usize, res: &mut HashSet<Vec<i32>>, nums: Vec<i32>){
            if i == opts.len(){
                res.insert(nums);
                return;
            }
            
            let mut nums2 = nums.clone();
            nums2.push(opts[i]);
            
            let j = i + 1;
            sub(opts, j, res, nums);
            sub(opts, j, res, nums2);
        }
        
        let mut res = HashSet::new();
        
        opts.sort_unstable();
        
        sub(&opts, 0, &mut res, vec![]);
        
        res.into_iter().collect::<Vec<Vec<i32>>>()
    }
}