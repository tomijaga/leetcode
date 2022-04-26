class Solution {
    public int[] getConcatenation(int[] nums) {
        int length = nums.length;
        
        int[] concat = new int[length * 2];
        
        for (int i = 0; i< length * 2; i++){
            concat[i] = nums[i % length];
        }
        
        return concat;
    }
}