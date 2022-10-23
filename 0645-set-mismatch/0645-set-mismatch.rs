use std::collections::HashSet;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        let mut unique_sum = 0;
        let mut total_sum = 0;
        
        let mut expected_sum = ((nums.len() * (nums.len() + 1))/ 2) as i32;
        
        for n in nums{
            if !set.contains(&n){
                unique_sum += n;
            }
            total_sum +=  n;
            set.insert(n);
        }
        
        vec![(total_sum - unique_sum), (expected_sum - unique_sum)]
    }
}