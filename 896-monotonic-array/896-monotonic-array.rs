impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut asc = None;
        for i in 0..nums.len()-1{
            if asc == None {
                if nums[i]> nums[i+1]{
                    asc = Some(false);
                }else if(nums[i]< nums[i+1]){
                    asc = Some(true)
                }
            }else{
                if let Some(is_asc) = asc{
                    if is_asc == true && nums[i] >nums[i+1]{
                        return false;
                    }else if (is_asc == false && nums[i] <nums[i+1]){
                        return false;
                    }
                }
            }
        }
        
        true
    }
}