impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut partition = 0;
        
        for i in (1..len).rev(){
            let a = nums[i - 1];
            let b = nums[i];
            
            if a == b{
                partition += 1;
                nums.swap(i, len - partition);
            }
            
        }
        
        nums[0..(len-partition)].sort_unstable();
        
        (len - partition) as i32
    }
}