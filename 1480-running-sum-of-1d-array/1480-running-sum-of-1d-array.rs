impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut sum = 0;
        
        nums.iter().map(|&a|{
            sum += a;
            sum
        }).collect::<Vec<i32>>()
    }
}