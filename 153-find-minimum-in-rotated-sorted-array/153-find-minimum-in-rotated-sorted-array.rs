impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut partition = nums[0];
        
        let mut min = 0;
        
        let (mut l, mut r) = (1, nums.len());
        
        while l < r {
            let mid = l + (r - l)/2;
            
            if nums[min] >= nums[mid] {
                min = mid;
                if partition >= nums[mid]{
                    r = mid;
                }else{
                    l = mid + 1;
                }
                
            }else if nums[min] < nums[mid]{
                if nums[mid] > partition{
                    l = mid +1;
                }else{
                    l = mid + 1;
                }
            }
             
        }
        
        nums[min]
    }
}