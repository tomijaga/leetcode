impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        
        let mut skipped = 0;
        let mut i = 1;
        while skipped < 2 && i < nums.len(){
            if nums[i- 1] > nums[i]{
                skipped +=1;
                if i >=2 && nums[i - 2] > nums[i]{
                    nums[i] = nums[i - 1];
                }else{
                    nums[i-1] = nums[i]
                }
            }
            i+=1;
        }
        
        skipped < 2
    }
}