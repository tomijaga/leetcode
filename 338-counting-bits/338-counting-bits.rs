impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut vec = vec![0];
        
        for i in 1..=n{
            vec.push(vec[(i/2) as usize ] + i % 2);
        }
        
        vec
    }
}