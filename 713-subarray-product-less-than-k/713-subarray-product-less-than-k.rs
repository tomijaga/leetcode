impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right, mut count, mut product) = (0, 0, 0, 1);
        
        for right in 0..nums.len() {
            product *= nums[right];
            
            while product >= k && left <= right {
                product /= nums[left];
                left += 1;
            }
            
            count += right - left + 1;
        }
        
        count as i32
    }
}