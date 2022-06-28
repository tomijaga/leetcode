use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(mut opts: Vec<i32>) -> Vec<Vec<i32>> {
        fn perm(mut opts: Vec<i32>, res: &mut HashSet<Vec<i32>>, nums: Vec<i32>){
            if opts.len() == 0{
                res.insert(nums);
                return;
            }
            
            for i in 0..opts.len(){
                let n = opts[i];
                let mut new_opts = opts.clone();
                new_opts.swap_remove(i);
                
                let mut new_nums = nums.clone();
                new_nums.push(n);
                perm(new_opts, res, new_nums);
            }
        }
        
        let mut res = HashSet::new();
        
        perm(opts, &mut res, vec![]);
        
        res.into_iter().collect::<Vec<Vec<i32>>>()
    }
}