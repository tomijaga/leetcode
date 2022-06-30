impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mid = nums[nums.len()/2];
        
        nums.into_iter().map(|n|{ (n - mid).abs() }).sum()
    }
}