impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        
        for n in nums.iter_mut(){
            sum += *n;
            *n = sum;
        }
    
        nums
    }
}