class Solution {
    public int[] productExceptSelf(int[] nums) {
        int[] res = new int[nums.length];
        
        Arrays.fill(res, 1);
        
        int prod = 1;
        
        for (int i = nums.length - 1; i>0; i-=1){
            prod *=nums[i];
            res[i-1] *= prod;
        }
        
        prod = 1;
        
        for (int i = 0; i<nums.length -1; i+=1){
            prod *=nums[i];
            res[i+1] *= prod;
        }
        
        return res;
    }
}