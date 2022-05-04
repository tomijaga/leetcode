impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        // [1, 1, 2, 2, 3, 4, 5]
        // ---------------------  (0 -> even, 1 -> odd)
        // [0, 1, 0, 1, -, 1, 0]
        
        while (left < right){
            let mid = left + (right - left)/2;
            let n = nums[mid];
            
            if mid % 2 == 0 {
                if (nums[mid + 1] == n){
                    left  = mid + 1;
                }else if mid > 0 && nums[mid - 1] == n{
                    right = mid - 1;
                }else{
                    return n;
                }
            }else{
                if (nums[mid + 1] == n){
                    right = mid - 1;
                }else if mid > 0 && nums[mid - 1] == n{
                    left  = mid + 1;
                }else{
                    return n;
                }
            }
        }   
        
        return nums[left];
    
    }
}