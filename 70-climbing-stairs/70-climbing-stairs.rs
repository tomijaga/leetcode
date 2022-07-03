impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut memo = vec![1, 2];
        
        for i in 2..n{
            memo.push(memo[i-1] + memo[i - 2])
        }
        
        memo[n - 1]
    }
}