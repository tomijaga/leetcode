impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums =nums;
        
        nums.sort_unstable();
        for i in 0..nums.len() -1{
            if (nums[i] == nums[i + 1]){
                return nums[i];
            }
        }

        unreachable!()
    }
}