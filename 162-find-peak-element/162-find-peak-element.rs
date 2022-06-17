impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        
        while l < r{
            let mid = l + (r - l)/2;
            
            let n1 = nums[mid];
            let n2 = nums[mid + 1];
            
            if n1 < n2{
                l = mid + 1;
            }else{
                r = mid;
            }
        }
        
        r as i32
    }
}