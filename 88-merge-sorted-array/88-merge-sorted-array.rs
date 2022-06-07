impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let mut op = m + n - 1;
        
        let (mut i, mut j) = (m - 1, n - 1);
        
        for ni in (0..nums1.len()).rev(){
            if (i == usize::MAX ){
                nums1[ni] = nums2[j];
                j-=1;
            }else if (j == usize::MAX ){
                nums1[ni] = nums1[i];
                i-=1;
            } else if (nums1[i] > nums2[j]){
                nums1[ni] = nums1[i];
                i-=1;
            }else{
                nums1[ni] = nums2[j];
                j-=1;
            }
        }
    }
}