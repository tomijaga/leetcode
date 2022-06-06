class Solution {
    public boolean isOdd(int n) {
        return n % 2 == 1;
    }
    
    public int singleNonDuplicate(int[] nums) {
        
        int l = 0;
        int r = nums.length - 1;
        
        // [1, 1, 2, 3, 3, 4, 4]
        // [0, 1, 0, 1, 0, 1, 0]
        
        while (l < r){
            int mid = l + (r - l)/2;
            
            int n = nums[mid];
            // System.out.println(l+ "-" + r + ", " +mid+ ":"+ n);
            if (isOdd(mid)){
                if ( nums[mid + 1] == n){
                    r = mid;
                }else{
                    l = mid + 1;
                }
            }else{
                if (nums[mid + 1] == n){
                    l = mid + 1;
                }else{
                    r =  mid;
                }
            }
        }
        
        return nums[r];
    }
    
}
