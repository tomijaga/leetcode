impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        
        nums.sort_unstable();
        let mut min = 1;
        for n in nums{
            if n == min{
                min+=1;
            }else if n>min{
                return min;
            }
        }
        return min;
    }
}