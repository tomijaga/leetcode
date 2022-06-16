impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 || target < nums[0]{
            return vec![-1, -1];
        }
        
        // binary search for left index
        let (mut l, mut r) = (0, nums.len() -1);
        while l < r{
            let mid = l + (r - l)/2;
            let n = nums[mid];
            
            if n < target{
                l = mid + 1;
            }else{
                r = mid;
            }
        }
        
        let left = l;
        
        // binary search for the right index
        r = nums.len() - 1;
        while (l <= r){
            let mid = (l + r)/2;
            let n = nums[mid];

            if n > target{
                r = mid - 1;
            }else{
                l = mid + 1;
            }
        }
        
        if nums[r] == target && nums[left] == target {
            vec![left as i32, r as i32]
        }else{
            vec![-1, -1]
        }
        
    }
}