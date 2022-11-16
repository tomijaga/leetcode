impl Solution {
    pub fn two_sum_less_than_k(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut max_sum = -1;
        
        for i in 0..nums.len(){
            for j in i + 1..nums.len(){
                let sum = nums[i] + nums[j];
                
                if sum < k{
                    max_sum = max_sum.max(sum);
                }
            }
        }
        
        max_sum
    }
}