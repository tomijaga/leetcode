impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        let len = nums.len();
        let acc_sum: i32 = nums.into_iter().sum();
        
        let sum: i32 = (len * (len + 1) / 2) as i32;
        
        sum - acc_sum
    }
}