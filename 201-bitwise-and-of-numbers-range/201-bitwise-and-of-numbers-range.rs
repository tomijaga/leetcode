impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut i = 0;
        
        while left < right {
            left >>= 1;
            right >>= 1;
            i +=1;
        }
        
        left.min(right) << i
    }
}