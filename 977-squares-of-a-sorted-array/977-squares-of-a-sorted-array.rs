impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = nums.len() - 1;
        
        let mut pos = nums.len() - 1;
        
        let mut res = vec![-4567; nums.len()];
        
        while i <= j && j != usize::MAX{
            let n1 = nums[i].pow(2);
            let n2 = nums[j].pow(2);
            
            if n1 > n2{
                res[pos] = n1;
                i+=1;
            }else{
                res[pos] = n2;
                j-=1;
            }
            pos-=1;
        }
        
        res
    }
}