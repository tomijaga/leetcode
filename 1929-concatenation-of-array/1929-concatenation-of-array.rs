impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        ans.extend(&nums);
        ans.extend(&nums);
        
        ans
    }
}