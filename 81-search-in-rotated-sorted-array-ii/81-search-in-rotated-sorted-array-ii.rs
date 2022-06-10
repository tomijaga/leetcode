impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
 
        let partition = nums[0];
        let (mut l, mut r) = (0, nums.len() - 1);
        
        // println!("len: {:?}", nums.len());
        
        while l < r{
            let mid = l + (r - l)/2;
            let n = nums[mid];
            
        // println!("[{:?}]({:?})", mid, nums[mid]);
            
            
            if target < partition{
                if n > partition{
                    l = mid + 1;
                }else if n < partition{
                    if n < target{
                        l = mid + 1;
                    }else if n > target{
                        r = mid;
                    }else{
                        return true;
                    }
                }else{
                    if n == nums[l]{
                        l+=1;
                    }  
                    if n == nums[r]{
                        r-=1;
                    } 
                }
            }else if target > partition {
                if n < partition{
                    r = mid;
                }else if n > partition{
                    if n < target{
                        l = mid + 1;
                    }else if n > target{
                        r = mid;
                    }else{
                        return true;
                    }
                }else{
                    if n == nums[l]{
                        l+=1;
                    }  
                    if n == nums[r]{
                        r-=1;
                    } 
                }
            }else{
                return true;
            }
        }
        // println!("[{:?}]({:?})", r, nums[r]);
        
        nums[r] == target
    }
}