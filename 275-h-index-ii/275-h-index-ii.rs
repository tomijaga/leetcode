impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut max = 0;
        let len = citations.len();
        for i in 0..len{
            let n = len - i;
            
            if citations[i] >= n as i32{
                return n as i32;
            }
        }
        
        0
    }
}