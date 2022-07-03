//  time complexity : O(n)
// space complexity : O(n)


impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1{
            return 1;
        }
        
        let n = n as usize;
        let mut memo = vec![0; n];
        
        memo[0] = 1;
        memo[1] = 2;
        
        for i in 2..n{
            memo[i] = memo[i-1] + memo[i - 2];
        }
        
        memo[n - 1]
    }
}