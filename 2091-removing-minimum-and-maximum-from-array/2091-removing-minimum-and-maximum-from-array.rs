use std::cmp;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min = (0 , i32::MAX);
        let mut max = (0 , i32::MIN);
        
        for (i, &n) in nums.iter().enumerate(){
            if n < min.1{
                min.0 = i;
                min.1 = n;
            }
            
            if n > max.1{
                max.0 = i;
                max.1 = n;
            }
            
        }

        // [-, -, d2, -, -, -, d1, -, -]
        
        let d1 = cmp::max(max.0 + 1, min.0 + 1);
        let d2 = cmp::min(max.0 + 1, min.0 + 1);
        
        let a = nums.len() - d2 + 1;
        let b = d1;
        
        let c = d2 + (nums.len() - d1 + 1);
        
        *([a, b, c].into_iter().min().unwrap()) as i32
        
    }
}