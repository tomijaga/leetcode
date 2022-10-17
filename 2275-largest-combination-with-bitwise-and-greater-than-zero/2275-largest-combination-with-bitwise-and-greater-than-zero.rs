impl Solution {
    pub fn largest_combination(mut candidates: Vec<i32>) -> i32 {
        let mut arr = vec![vec![0_i32;candidates.len()];32];
        let mut sum = 0;
        let mut max_len = 0;
        
        for _ in 0..32{
            for n in candidates.iter_mut(){
                sum += 1 & *n;
                *n = *n >> 1;
            }    
            
            max_len = max_len.max(sum);
            sum = 0;
        }
        
        max_len
    }
}