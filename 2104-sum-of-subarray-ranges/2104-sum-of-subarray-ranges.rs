use std::collections::BTreeSet;
use std::cmp::Reverse;

impl Solution {
    pub fn sub_array_ranges(mut nums: Vec<i32>) -> i64 {
        let mut sum = 0;
        let mut set = BTreeSet::new();
        
        fn get_min(set: &BTreeSet<i32>) -> i32{
            *set.iter().next().unwrap()
        }
        
        fn get_max(set: &BTreeSet<i32>) -> i32{
            *set.iter().rev().next().unwrap()
        }
        
        for i in 0..nums.len(){
            set.insert(nums[i]);
            
            for j in i + 1..nums.len(){
                set.insert(nums[j]);
                sum+= (get_max(&set) - get_min(&set)) as i64;
            }
            
            set.clear();
        }
        
        sum
    }
}