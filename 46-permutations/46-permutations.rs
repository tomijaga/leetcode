impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(first: usize, res: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>){
            if first == nums.len(){
                res.push(nums.clone());
                return;
            }
            
            for i in first..nums.len(){
                nums.swap(first, i);
                
                backtrack(first + 1, res, nums);
                
                nums.swap(first, i);
            }
        }
        
        let mut res = vec![];
        
        backtrack(0, &mut res, &mut nums);
        
        res
    }
}