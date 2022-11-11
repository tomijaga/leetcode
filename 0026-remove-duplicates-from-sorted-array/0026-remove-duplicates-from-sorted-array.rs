impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        
        for i in (1..len).rev(){
            let a = nums[i - 1];
            let b = nums[i];
            
            if a == b{
                nums.swap(i, len - 1);
                nums.pop();
                len-=1;
            }
        }
        
        nums.sort_unstable();
        
        nums.len() as i32
    }
}