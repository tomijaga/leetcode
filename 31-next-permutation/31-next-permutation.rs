use std::collections::BinaryHeap;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let (mut i, mut j) = (nums.len() - 2, nums.len() - 1);
        let mut heap = BinaryHeap::new();
        
        for v in nums.windows(2).rev(){
            let n1 = v[0];
            let n2 = v[1];
            
            if (n1 < n2){
                
                for k in j..nums.len(){
                    if nums[k] > nums[i]{
                        heap.push((-nums[k], k));
                    }
                }
                
                let (_, j) = heap.pop().unwrap();
                
                nums.swap(i, j);
                break;
            }
            
            i-=1;
            j-=1;
        }
        
        nums[i+1..].reverse();
    }
}