impl Solution {
    pub fn subsets(opts: Vec<i32>) -> Vec<Vec<i32>> {
        
        pub fn sub(opts: &Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>){
            if i == opts.len(){
                res.push(nums.to_vec());
                return;
            }
            
            let j = i + 1;
            sub(opts, j, res, nums);
            
            nums.push(opts[i]);
            sub(opts, j, res, nums);
            nums.pop();
            
        }
        
        let mut res = vec![];
        let mut nums = vec![];
        
        sub(&opts, 0, &mut res, &mut nums);
        
        res
    }
}