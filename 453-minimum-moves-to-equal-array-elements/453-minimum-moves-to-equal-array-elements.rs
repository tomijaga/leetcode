impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter().sum::<i32>() - (nums.iter().min().unwrap() * nums.len() as i32)
    }
}