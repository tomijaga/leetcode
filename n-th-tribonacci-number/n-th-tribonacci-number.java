class Solution {
    public int tribonacci(int n) {
        if (n < 2){
            return n;
        }else if (n == 2){
            return 1;
        }
        
        int[] cache = new int[n + 1];
        cache[0] = 0;
        cache[1] = 1;
        cache[2] = 1;
        
        for (int i = 3; i<=n; i+=1){
            cache[i] = cache[i-1] + cache[i-2] + cache[i-3];
        }
        
        return cache[n];
    }
}