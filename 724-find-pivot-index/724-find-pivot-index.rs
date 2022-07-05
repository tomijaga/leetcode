impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        
        let mut l = 0;
        let mut r: i32 = nums.iter().sum();
        
        for (i, n) in nums.into_iter().enumerate(){
            r -= n;
            
            if l == r{
                return i as i32;
            }
            
            l += n;
        }
        
        -1
    }
}