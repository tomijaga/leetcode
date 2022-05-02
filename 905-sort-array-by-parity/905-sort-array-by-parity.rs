use std::cmp::Ordering;

pub fn is_even(n: &i32)->bool{
    n%2 == 0
}

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        
        nums.sort_by(|a, b|{ 
            if (is_even(a) == is_even(b)){
                a.cmp(b)
            }else if (is_even(a)){
               Ordering::Less 
            }else {
                Ordering::Greater
            }
        });
        
        nums
    }
}