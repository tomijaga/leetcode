use std::cmp;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len();
        
        let mut l = 0;
        let mut r = len;
        
        let mut max = 0;
        
        while (l < r){
            let mid = l + (r - l)/2;
            let target = len - mid;
            
            let citation = citations[mid];
            if citations[mid] > target as i32{
                r = mid;
                max = cmp::max(max, target as i32);
            }else if citations[mid] < target as i32{
                l = mid + 1;
            }else{
                return citations[mid]
            }
        }
        
        (len - r ) as i32
    }
}