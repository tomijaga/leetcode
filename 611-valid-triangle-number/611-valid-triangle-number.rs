impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        let mut count = 0;
        for k in (2..nums.len()).rev(){
            let mut i = 0;
            let mut j = k - 1;
            
            while (i< j){
                let sides = [nums[i], nums[j], nums[k]];
                
                if nums[k] < nums[j]+ nums[i]{
                    count += j - i;
                    j-=1;
                }else{
                    i+=1;
                }
            }
        }
        
        count as i32
    }
}
