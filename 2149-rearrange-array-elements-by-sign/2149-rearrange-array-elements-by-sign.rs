
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut res = vec![0; nums.len()];
        
        let mut pos = 0;
        let mut neg = 1;
        
        for n in nums{
            if n >= 0{
                res[pos] = n;
                pos +=2;
            }else{
                res[neg] = n;
                neg +=2;
            }
        }
        
        res
    }
}