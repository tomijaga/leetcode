use std::cmp;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut max = 0;
        
        for (i, &n) in nums.iter().enumerate(){
            if n < nums[min]{
                min = i;
            }
            
            if n > nums[max]{
                max = i;
            }
            
        }

        // [-, -, d2, -, -, -, d1, -, -]
        
        let d1 = cmp::max(max + 1, min + 1);
        let d2 = cmp::min(max + 1, min + 1);
        
        let a = nums.len() - d2 + 1;
        let b = d1;
        
        let c = d2 + (nums.len() - d1 + 1);
        
        *([a, b, c].into_iter().min().unwrap()) as i32
        
    }
}