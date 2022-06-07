impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let op = m + n;
        
        let (mut i, mut j) = (0, 0);
        
        let n1_copy = nums1[0..m].to_vec();
        
        for num in nums1.iter_mut(){
            if (i >= m){
                *num = nums2[j];
                j+=1;
            }else if (j >=n){
               *num = n1_copy[i];
                i+=1;
            } else if (n1_copy[i]< nums2[j]){
                *num = n1_copy[i];
                i+=1;
            }else{
                *num = nums2[j];
                j+=1;
            }
        }
    }
}