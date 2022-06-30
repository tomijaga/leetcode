impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        
        let sum: i32 = nums.iter().sum();
        let min_sum = (min * nums.len() as i32);
        
        sum - min_sum
    }
}