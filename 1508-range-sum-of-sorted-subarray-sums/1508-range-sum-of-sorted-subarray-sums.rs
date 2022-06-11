use std::collections::BinaryHeap;

impl Solution {
    pub fn range_sum(mut nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        
        let mut prev = nums[0];
        nums[0] = 0;
        for i in 1..nums.len(){
            let tmp = prev;
            prev = nums[i];
            
            nums[i] = nums[i - 1] + tmp;
        }
        
        nums.push(prev + *nums.last().unwrap());
        
        let mut heap = BinaryHeap::new();
        
        for i in 0..nums.len(){
            for j in i+1..nums.len(){
                heap.push(nums[j] - nums[i]);
                
                if heap.len() > right{
                    heap.pop();
                }
            }
        }
        
        // println!("{:?}", &heap);
//         println!("{:?}", heap.peek());
//         println!("{:?}", heap.pop());
//         println!("{:?}", heap.peek());
        
        let mut sum: i64 = 0;
        for _ in left..=right{
            sum += heap.pop().unwrap() as i64;
        }
        
        (sum % (10_i64.pow(9) + 7)) as i32
    }
}