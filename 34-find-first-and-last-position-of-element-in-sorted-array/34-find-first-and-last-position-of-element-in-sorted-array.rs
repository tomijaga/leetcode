impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0{
            return vec![-1, -1];
        }
        
        let (mut l, mut r) = (0, nums.len() -1);
        while l < r{
            let mid = l + (r - l)/2;
            let n = nums[mid];
            
            if n < target{
                l = mid + 1;
            }else if n > target{
                r = mid;
            }else{
                r = mid;
                break;
            }
        }
        
        if nums[r] != target {
            return vec![-1, -1];
        }
        
        let mut target_index = r;
        let (mut l, mut r) = (0, target_index);
        
        while (l < r){
            let mid = l + (r - l)/2;
            let n = nums[mid];

            if n < target{
                l = mid + 1;
            }else{
                r = mid;
            }
        }

        let left = r;
        let (mut l, mut r) = (target_index, nums.len() - 1);
        
        while (l <= r){
            let mid = (l + r)/2;
            let n = nums[mid];

            if n > target{
                r = mid - 1;
            }else{
                l = mid + 1;
                
            }
        }
        
        vec![left as i32, r as i32]
    }
}