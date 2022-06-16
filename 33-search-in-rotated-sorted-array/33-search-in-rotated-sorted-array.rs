impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut first = nums[0];
        
        while l <= r{
            let mid = l + (r-l)/2;
            let n = nums[mid];
            
            if target < first{
                if n < target || n >= first{
                    l = mid +1;
                }else if n > target{
                    r = mid - 1;
                }else{
                    return mid as i32;
                }
            }else if target > first{
                if n > target || n < first{
                    r = mid - 1;
                }else if n < target{
                    l = mid + 1;
                }else{
                    return mid as i32;
                }
            }else{
                return 0;
            }
        }
        
        -1
    }
}