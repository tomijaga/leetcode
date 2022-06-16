impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len()-1;
        
        while (l < r){
            let mid = l + (r - l)/2;
            let n = nums[mid];
            
            if n > target{
                r = mid;
            }else if n < target{
                l = mid +1;
            }else{
                return mid as i32;
            }
        }
        
        if nums[r] == target{
            r as i32
        }else{
            -1
        }
    }
}