impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut valid_indices = 0;
        
        for i in (1..len){
            let a = nums[i - 1];
            let b = nums[i];
            
            if a != b{
                valid_indices += 1;
                nums[valid_indices] = b;
            }
        }
        
        (valid_indices + 1) as i32
    }
}