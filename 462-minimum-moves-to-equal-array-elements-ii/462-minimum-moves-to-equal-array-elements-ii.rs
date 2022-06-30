impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        
        let (mut i, mut j) = (0, nums.len() - 1);
        
        let mut d = 0;
        
        while i < j{
            d += (nums[j] - nums[i]);
            i+=1;
            j-=1;
        }
        
        d
    }
}