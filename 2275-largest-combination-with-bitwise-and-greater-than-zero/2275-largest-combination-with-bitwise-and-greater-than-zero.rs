impl Solution {
    pub fn largest_combination(mut candidates: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_len = 0;
        
        for pos in 0..32{
            for &n in candidates.iter(){
                sum += 1 & (n >> pos);
            }    
            
            max_len = max_len.max(sum);
            sum = 0;
        }
        
        max_len
    }
}