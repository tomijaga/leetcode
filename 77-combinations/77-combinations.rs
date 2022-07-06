impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn cbns(start: i32, end: i32, res:&mut Vec<Vec<i32>>, nums: &mut Vec<i32>, k: usize){
            if nums.len() == k{
                res.push(nums.clone());
                return;
            }
            
            
            for n in start..=end{
                nums.push(n);
                cbns(n + 1, end, res, nums, k);
                nums.pop();
            }
        }  
        
        let mut res = vec![];
        let mut nums = vec![];
        
        cbns(1, n, &mut res, &mut nums, k as usize);
        
        res
    }
}