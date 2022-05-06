use rand::prelude::*;

struct Solution {
    nums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self{
            nums
        }
    }
    
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }
    
    fn shuffle(&self) -> Vec<i32> {
        let mut clone = self.nums.clone();
        let mut rng = rand::thread_rng();
        
        clone.shuffle(&mut rng);
        clone
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */