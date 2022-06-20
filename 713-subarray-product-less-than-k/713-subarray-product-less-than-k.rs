impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {

        let mut subs = 0;
        
        let mut i = 0;
        let mut j = 1;
        
        let mut prod = nums[i];
        
        while i< j && j <= nums.len(){
            let n = nums[j - 1];
            
            if prod >= k{
                prod /= nums[i];
                i+=1;
                if i==j && i < nums.len(){
                    j+=1;
                    prod = nums[i];
                }
                
            }else {
                subs+= (j - i) as i32;
                j+=1;
                
                if j<= nums.len(){
                    prod *= nums[j - 1];
                }
            }
        }
        
        subs
    }
}