impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut longest = 0;
        for num in &set {
            if !set.contains(&(*num - 1)) {
                let mut curr = *num;
                let mut longer = 1;
                while set.contains(&(curr + 1)) {
                    curr += 1;
                    longer += 1;
                }
                longest = std::cmp::max(longest, longer)
            }
        }
        longest
    }
}