impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        
        let len = nums.len();
        
        let mut start = len;
        let mut end = 0;
        
        println!("{:?}\n{:?}", &nums, &sorted);
        for i in 0..len{
            if nums[i]!= sorted[i]{
                start = i;
                break;
            }
        }
        
        
        for j in (0..len).rev(){
            if nums[j]!= sorted[j]{
                end = j;
                break;
            }
        }
        
        if end < start{
            0
        }else{
            (end - start + 1 ) as i32
        }
    }
}