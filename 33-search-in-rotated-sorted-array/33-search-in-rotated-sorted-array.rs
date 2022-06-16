impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut start = nums[0];
        let mut min = start;
        
        while l < r{
            let mid = l + (r-l)/2;
            
            let n = nums[mid];
            
            if n < min{
                min = n;
                r = mid;
            }else{
                l = mid + 1;
            }
        }
        
        let mut partition = r;
        // println!("{:?}", partition);
        let a = nums[..partition].binary_search(&target);
        let b = nums[partition..].binary_search(&target);
        
        if a.is_ok(){
            return a.unwrap() as i32;
        }
        
         if b.is_ok(){
            return (b.unwrap() + partition) as i32;
        }
        
        -1
    }
}