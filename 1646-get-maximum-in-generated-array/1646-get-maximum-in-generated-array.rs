impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n < 2{
            return n;
        }
        
        let mut nums = vec![0; (n + 1) as usize];
        
        nums[1] = 1;
        let mut max_val = 1;
        
        for i in 1..=(n/2) as usize{
            
            let (i1, i2) = (i * 2, (i * 2) + 1);
            nums[i1] = nums[i];
            if i2 <= n as usize{
                nums[i2] = nums[i] + nums[i + 1];
                max_val = max_val.max(nums[i1]).max(nums[i2]);
            }else{
                max_val = max_val.max(nums[i1]);
            }
        }
        
        max_val
    }
}