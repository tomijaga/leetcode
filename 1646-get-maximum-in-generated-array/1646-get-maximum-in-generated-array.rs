impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0{
            return 0;
        }
        
        let mut nums = vec![0, 1];
        let mut max = 1;
        
        for i in 2..=(n) as usize{
            let n = if i % 2 == 0{
                nums[i/2]
            }else{
                nums[i/2] + nums[i/2 + 1]
            };
            
            max = n.max(max);
            nums.push(n);
        }
        
        max
    }
}